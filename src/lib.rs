use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::convert::identity;
use std::ffi::CStr;
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;

use pgx::{AllocatedByPostgres, AnyArray, Array, FromDatum, IntoDatum, pg_sys as pg, PgBox, PgRelation, TryFromDatumError};
use pgx::pg_sys::{HeapTupleData, LogicalDecodingContext, Oid, Point};
use pgx::prelude::*;
use prost::Message;

use crate::pboutput::{Begin, ChangeType, ColumnData, IntArray};
use crate::pboutput::column_data::Value;
use crate::pboutput::message::Item;

pub mod pboutput;
pub mod types;

pgx::pg_module_magic!();

#[no_mangle]
pub unsafe extern "C" fn _PG_output_plugin_init(cb: *mut pg::OutputPluginCallbacks) {
    (*cb).startup_cb = Some(startup);
    (*cb).begin_cb = Some(begin);
    (*cb).change_cb = Some(change);
    (*cb).commit_cb = Some(commit);
    (*cb).shutdown_cb = Some(shutdown);
}

#[derive(Debug, Default)]
struct State {
    unsent_begin: Option<Begin>,
    sent_rels: HashMap<Oid, u64>,
}

#[pg_guard]
unsafe extern "C" fn startup(
    ctx: *mut pg::LogicalDecodingContext,
    options: *mut pg::OutputPluginOptions,
    _is_init: bool,
) {
    log!("Startup");
    (*ctx).output_plugin_private = PgBox::new(State::default()).as_ptr() as *mut _;
    (*options).output_type = pg::OutputPluginOutputType_OUTPUT_PLUGIN_BINARY_OUTPUT;
}

unsafe fn write_msg(ctx: *mut pg::LogicalDecodingContext, msg: pboutput::Message, is_final: bool) {
    write_buf(ctx, &msg.encode_to_vec(), is_final);
}

unsafe fn write_buf(ctx: *mut pg::LogicalDecodingContext, data: &[u8], is_final: bool) {
    pg::OutputPluginPrepareWrite(ctx, is_final);
    pg::appendBinaryStringInfoNT((*ctx).out, data.as_ptr() as *const _, data.len() as _);
    pg::OutputPluginWrite(ctx, is_final);
}

#[pg_guard]
unsafe extern "C" fn begin(
    ctx: *mut pg::LogicalDecodingContext,
    txn: *mut pg::ReorderBufferTXN,
) {
    log!("Begin");
    let ctx = ctx.as_mut().unwrap();
    let state = (ctx.output_plugin_private as *mut State).as_mut().unwrap();
    state.unsent_begin = Some(pboutput::Begin {
        start_lsn: (*txn).first_lsn,
        final_lsn: (*txn).final_lsn,
        commit_time: 0,
        // commit_time: (*txn).commit_time as _,
        txid: (*txn).xid,
    });
}


#[pg_guard]
unsafe extern "C" fn change(
    ctx: *mut pg::LogicalDecodingContext,
    _txn: *mut pg::ReorderBufferTXN,
    relation: pg::Relation,
    change: *mut pg::ReorderBufferChange,
) {
    log!("Change");
    let ctx = ctx.as_mut().unwrap();
    let state = (ctx.output_plugin_private as *mut State).as_mut().unwrap();
    if let Some(begin) = state.unsent_begin.take() {
        write_msg(ctx, pboutput::Message {
            item: Some(Item::Begin(begin))
        }, false)
    }

    let relation = relation.as_mut().unwrap();
    let relation = &PgRelation::from_pg(relation);

    maybe_sent_relation(state, ctx, relation);

    let change = change.as_mut().unwrap();
    if change.action == pg::ReorderBufferChangeType_REORDER_BUFFER_CHANGE_TRUNCATE {
        write_msg(ctx, pboutput::Message {
            item: Some(Item::Truncate(pboutput::Truncate {
                relid: relation.rd_id
            }))
        }, true);
        return;
    }

    let new = change.data.tp.newtuple.as_mut();
    let old = change.data.tp.oldtuple.as_mut();

    log!("Change: action");
    let old = old.map(|tuple| convert_tuple(relation, &mut tuple.tuple)).unwrap_or_else(|| Ok(vec![])).unwrap();
    let new = new.map(|tuple| convert_tuple(relation, &mut tuple.tuple)).unwrap_or_else(|| Ok(vec![])).unwrap();

    let typ = match change.action {
        pg::ReorderBufferChangeType_REORDER_BUFFER_CHANGE_INSERT => ChangeType::Insert,
        pg::ReorderBufferChangeType_REORDER_BUFFER_CHANGE_DELETE => ChangeType::Delete,
        pg::ReorderBufferChangeType_REORDER_BUFFER_CHANGE_UPDATE => ChangeType::Update,
        _ => panic!(),
    };

    let out = pboutput::Change {
        old,
        new,
        relid: relation.rd_id,
        r#type: typ as _,
    };

    let out = pboutput::Message {
        item: Some(Item::Change(out))
    };
    write_msg(ctx, out, true)
}

unsafe fn maybe_sent_relation(state: &mut State, ctx: &mut LogicalDecodingContext, relation: &PgRelation) {
    let mut h = DefaultHasher::new();
    relation.tuple_desc().iter().for_each(|v| v.type_oid().hash(&mut h));
    let h = h.finish();

    match state.sent_rels.get(&relation.oid()) {
        None => {
            send_relation(ctx, relation);
            state.sent_rels.insert(relation.rd_id, h);
        }
        Some(v) if *v != h => {
            send_relation(ctx, relation);
            state.sent_rels.insert(relation.rd_id, h);
        }
        _ => {}
    }
}

unsafe fn send_relation(ctx: &mut LogicalDecodingContext, relation: &PgRelation) {
    log!("SendRelation");
    let rd_rel = relation.rd_rel.as_ref().unwrap();
    let rd_att = relation.rd_att.as_ref().unwrap();
    let namespace = CStr::from_ptr(pg::get_namespace_name(rd_rel.relnamespace));
    let name = CStr::from_ptr(pg::get_rel_name(relation.rd_id));

    let mut columns = vec![];
    for i in 0..rd_att.natts {
        let attr = &*rd_att.attrs.as_ptr().offset(i as isize);
        let num = attr.attnum;

        if attr.is_dropped() || attr.num() <= 0 {
            continue;
        }
        let col = CStr::from_ptr(pg::get_attname(relation.rd_id, num, false));
        let typ = pg::get_atttype(relation.rd_id, num);

        let mapped_type = crate::types::map_oid_to_type(typ);

        columns.push(pboutput::Column {
            is_key: false,
            name: col.to_str().unwrap().to_string(),
            r#type: Some(mapped_type),
        });
    }
    let rel = pboutput::Relation {
        id: relation.rd_id,
        namespace: namespace.to_str().unwrap().to_string(),
        name: name.to_str().unwrap().to_string(),
        replica_identity: 0,
        columns,
    };
    let msg = pboutput::Message {
        item: Some(Item::Relation(rel))
    };
    write_msg(ctx, msg, false);
}


unsafe fn convert_val<T: FromDatum  + 'static>(
    tuple: &PgHeapTuple<AllocatedByPostgres>,
    i: NonZeroUsize,
    wrap: impl Fn(T) -> Value,
) -> Result<Value, TryFromDatumError> {
    Ok(tuple.get_by_index::<T>(i)?.map(wrap).unwrap_or_else(|| Value::Null(Default::default())))
}

unsafe fn convert_tuple(relation: &PgRelation, tuple: &mut HeapTupleData) -> Result<Vec<pboutput::ColumnData>, TryFromDatumError> {
    log!("ConvertTuple");
    let desc = pgx::PgTupleDesc::from_relation(relation);
    let tuple = PgHeapTuple::from_heap_tuple(desc, tuple);
    let tuple = &tuple;

    log!("ConvertTuple - 2");
    let mut res: Vec<pboutput::ColumnData> = vec![];
    for (i, att) in tuple.attributes() {
        let PgOid::BuiltIn(bt) = att.type_oid() else {
            res.push(ColumnData {
                value: Some(Value::Null(Default::default()))
            });
            continue;
        };
        log!("ConvertTuple - 3");
        let value = match bt {
            PgBuiltInOids::BOOLOID => convert_val::<bool>(tuple, i, |v| Value::Bool(v))?,
            PgBuiltInOids::INT2OID => convert_val::<i16>(tuple, i, |v| Value::Int(v as _))?,
            PgBuiltInOids::INT4OID => convert_val::<i32>(tuple, i, |v| Value::Int(v as _))?,
            PgBuiltInOids::INT8OID => convert_val::<i64>(tuple, i, |v| Value::Int(v as _))?,
            PgBuiltInOids::FLOAT4OID => convert_val::<f32>(tuple, i, |v| Value::Float(v as _))?,
            PgBuiltInOids::FLOAT8OID => convert_val::<f64>(tuple, i, |v| Value::Float(v as _))?,
            PgBuiltInOids::TEXTOID => convert_val::<String>(tuple, i, |v| Value::Text(v))?,
            PgBuiltInOids::BYTEAOID => convert_val::<Vec<u8>>(tuple, i, |v| Value::Binary(v))?,
            PgBuiltInOids::POINTOID => convert_val::<Point>(tuple, i, |v| Value::Point(pboutput::Point {
                x: v.x,
                y: v.y,
            }))?,

            PgBuiltInOids::INT2ARRAYOID => convert_val::<Array<i16>>(tuple, i, |v| {
                Value::IntArray(IntArray {
                    items:v.into_iter().filter_map(identity)
                        .map(|v| v as _).collect()
                })
            })?,

            PgBuiltInOids::INT4ARRAYOID => convert_val::<AnyArray>(tuple, i, |v| {
                Value::IntArray(IntArray {
                    items: AnyArray::into::<Array<i32>>(&v).unwrap().into_iter().filter_map(identity)
                        .map(|v| v as _).collect()
                })
            })?,
            PgBuiltInOids::INT8ARRAYOID => convert_val::<AnyArray>(tuple, i, |v| {
                Value::IntArray(IntArray {
                    items: AnyArray::into::<Array<i64>>(&v).unwrap().into_iter().filter_map(identity)
                        .map(|v| v as _).collect()
                })
            })?,
            _ => todo!(),
        };
        res.push(ColumnData {
            value: Some(value),
        });
    }

    Ok(res)
}

unsafe extern "C" fn commit(
    ctx: *mut pg::LogicalDecodingContext,
    txn: *mut pg::ReorderBufferTXN,
    _lsn: pg::XLogRecPtr,
) {
    log!("Commit");
    let txn = &*txn;
    let ctx = ctx.as_mut().unwrap();
    let state = (ctx.output_plugin_private as *mut State).as_mut().unwrap();

    // Do not send empty begin & commit
    if let Some(_) = &state.unsent_begin {
        return;
    }
    write_msg(ctx, pboutput::Message {
        item: Some(Item::Commit(pboutput::Commit {
            final_lsn: txn.final_lsn,
            #[cfg(feature = "pg15")]
            commit_time: txn.xact_time.commit_time,
        }))
    }, true);
}

unsafe extern "C" fn shutdown(ctx: *mut pg::LogicalDecodingContext) {
    log!("Shutdown");
    let ctx = &mut *ctx;
    let _ = PgBox::<State>::from_pg(ctx.output_plugin_private as *mut _);
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::prelude::*;

    fn spi_get_changes() -> Vec<(i64, u32, Vec<u8>)> {
        let query = "SELECT pg_logical_slot_peek_binary_changes('blog_slot', NULL, NULL)";

        let mut results = Vec::new();

        Spi::connect(|client| {
            let mut tup_table: pgx::SpiTupleTable = client.select(query, None, None);

            while let Some(row) = tup_table.next() {
                let a: i64 = row["location"].value().unwrap();
                let b: u32 = row["xid"].value().unwrap();
                let c: Vec<u8> = row["data"].value().unwrap();
                results.push((a, b, c));
            }

            Ok(Some(()))
        });

        results
    }

    #[pg_test]
    fn test_hello_aaaa() {
        Spi::run(r#"SELECT * FROM pg_create_logical_replication_slot('blog_slot', 'pboutput');"#);
        Spi::run(r#"CREATE TABLE a(a integer)"#);
        Spi::run(r#"INSERT INTO a(a) values (0)"#);
        // Spi::run(r#"COMMIT;"#);
        panic!("CHANGES: {:?}", spi_get_changes());
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![
            "wal_level = logical"
        ]
    }
}

