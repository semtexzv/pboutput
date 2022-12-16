#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Begin {
    #[prost(uint64, tag = "1")]
    pub start_lsn: u64,
    #[prost(uint64, tag = "2")]
    pub final_lsn: u64,
    #[prost(uint64, tag = "3")]
    pub commit_time: u64,
    #[prost(uint32, tag = "4")]
    pub txid: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(uint64, tag = "2")]
    pub final_lsn: u64,
    #[prost(int64, tag = "3")]
    pub commit_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Column {
    #[prost(bool, tag = "1")]
    pub is_key: bool,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof = "column::Type", tags = "3, 4")]
    pub r#type: ::core::option::Option<column::Type>,
}
/// Nested message and enum types in `Column`.
pub mod column {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(enumeration = "super::BuiltinType", tag = "3")]
        Builtin(i32),
        #[prost(uint32, tag = "4")]
        Custom(u32),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "ReplicaIdentity", tag = "4")]
    pub replica_identity: i32,
    #[prost(message, repeated, tag = "5")]
    pub columns: ::prost::alloc::vec::Vec<Column>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Null {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(double, tag = "1")]
    pub x: f64,
    #[prost(double, tag = "2")]
    pub y: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolArray {
    #[prost(bool, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntArray {
    #[prost(int64, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatArray {
    #[prost(double, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextArray {
    #[prost(string, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesArray {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnData {
    ///   string name = 1;
    #[prost(
        oneof = "column_data::Value",
        tags = "2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14"
    )]
    pub value: ::core::option::Option<column_data::Value>,
}
/// Nested message and enum types in `ColumnData`.
pub mod column_data {
    ///   string name = 1;
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(message, tag = "2")]
        Null(super::Null),
        #[prost(bool, tag = "3")]
        Bool(bool),
        #[prost(int64, tag = "4")]
        Int(i64),
        #[prost(double, tag = "5")]
        Float(f64),
        #[prost(string, tag = "6")]
        Text(::prost::alloc::string::String),
        #[prost(bytes, tag = "7")]
        Binary(::prost::alloc::vec::Vec<u8>),
        #[prost(message, tag = "8")]
        Point(super::Point),
        #[prost(message, tag = "10")]
        BoolArray(super::BoolArray),
        #[prost(message, tag = "11")]
        IntArray(super::IntArray),
        #[prost(message, tag = "12")]
        FloatArray(super::FloatArray),
        #[prost(message, tag = "13")]
        TextArray(super::TextArray),
        #[prost(message, tag = "14")]
        BytesArray(super::BytesArray),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    #[prost(uint32, tag = "1")]
    pub relid: u32,
    #[prost(message, repeated, tag = "2")]
    pub old: ::prost::alloc::vec::Vec<ColumnData>,
    #[prost(message, repeated, tag = "3")]
    pub new: ::prost::alloc::vec::Vec<ColumnData>,
    #[prost(enumeration = "ChangeType", tag = "4")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Truncate {
    #[prost(uint32, tag = "1")]
    pub relid: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    #[prost(oneof = "message::Item", tags = "1, 2, 3, 4, 5, 6")]
    pub item: ::core::option::Option<message::Item>,
}
/// Nested message and enum types in `Message`.
pub mod message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Item {
        #[prost(message, tag = "1")]
        Begin(super::Begin),
        #[prost(message, tag = "2")]
        Commit(super::Commit),
        #[prost(message, tag = "3")]
        Relation(super::Relation),
        #[prost(message, tag = "4")]
        Type(super::Type),
        #[prost(message, tag = "5")]
        Change(super::Change),
        #[prost(message, tag = "6")]
        Truncate(super::Truncate),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageSet {
    #[prost(message, repeated, tag = "1")]
    pub msg: ::prost::alloc::vec::Vec<Message>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReplicaIdentity {
    None = 0,
}
impl ReplicaIdentity {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReplicaIdentity::None => "None",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BuiltinType {
    Missing = 0,
    Bool = 16,
    Bytea = 17,
    Char = 18,
    Name = 19,
    Int8 = 20,
    Int2 = 21,
    Int2vector = 22,
    Int4 = 23,
    Regproc = 24,
    Text = 25,
    Oid = 26,
    Tid = 27,
    Xid = 28,
    Cid = 29,
    Oidvector = 30,
    Json = 114,
    Xml = 142,
    Xid8 = 5069,
    Point = 600,
    Lseg = 601,
    Path = 602,
    Box = 603,
    Polygon = 604,
    Line = 628,
    Float4 = 700,
    Float8 = 701,
    Unknown = 705,
    Circle = 718,
    Macaddr = 829,
    Inet = 869,
    Cidr = 650,
    Macaddr8 = 774,
    Aclitem = 1033,
    Bpchar = 1042,
    Varchar = 1043,
    Date = 1082,
    Time = 1083,
    Timestamp = 1114,
    Timestamptz = 1184,
    Interval = 1186,
    Timetz = 1266,
    Bit = 1560,
    Varbit = 1562,
    Numeric = 1700,
    Refcursor = 1790,
    Regprocedure = 2202,
    Regoper = 2203,
    Regoperator = 2204,
    Regclass = 2205,
    Regcollation = 4191,
    Regtype = 2206,
    Regrole = 4096,
    Regnamespace = 4089,
    Uuid = 2950,
    Tsvector = 3614,
    Gtsvector = 3642,
    Tsquery = 3615,
    Regconfig = 3734,
    Regdictionary = 3769,
    Jsonb = 3802,
    Jsonpath = 4072,
    TxidSnapshot = 2970,
    PgSnapshot = 5038,
    Int4range = 3904,
    Numrange = 3906,
    Tsrange = 3908,
    Tstzrange = 3910,
    Daterange = 3912,
    Int8range = 3926,
    Record = 2249,
    Recordarray = 2287,
    Cstring = 2275,
    Any = 2276,
    Anyarray = 2277,
    Void = 2278,
    Trigger = 2279,
    LanguageHandler = 2280,
    Internal = 2281,
    Anyelement = 2283,
    Anynonarray = 2776,
    Anyenum = 3500,
    FdwHandler = 3115,
    IndexAmHandler = 325,
    TsmHandler = 3310,
    TableAmHandler = 269,
    Anyrange = 3831,
    Anycompatible = 5077,
    Anycompatiblearray = 5078,
    Anycompatiblenonarray = 5079,
    Anycompatiblerange = 5080,
    Boolarray = 1000,
    Byteaarray = 1001,
    Chararray = 1002,
    Namearray = 1003,
    Int8array = 1016,
    Int2array = 1005,
    Int2vectorarray = 1006,
    Int4array = 1007,
    Regprocarray = 1008,
    Textarray = 1009,
    Oidarray = 1028,
    Tidarray = 1010,
    Xidarray = 1011,
    Cidarray = 1012,
    Oidvectorarray = 1013,
    Jsonarray = 199,
    Xmlarray = 143,
    Xid8array = 271,
    Pointarray = 1017,
    Lsegarray = 1018,
    Patharray = 1019,
    Boxarray = 1020,
    Polygonarray = 1027,
    Linearray = 629,
    Float4array = 1021,
    Float8array = 1022,
    Circlearray = 719,
    Moneyarray = 791,
    Macaddrarray = 1040,
    Inetarray = 1041,
    Cidrarray = 651,
    Macaddr8array = 775,
    Aclitemarray = 1034,
    Bpchararray = 1014,
    Varchararray = 1015,
    Datearray = 1182,
    Timearray = 1183,
    Timestamparray = 1115,
    Timestamptzarray = 1185,
    Intervalarray = 1187,
    Timetzarray = 1270,
    Bitarray = 1561,
    Varbitarray = 1563,
    Numericarray = 1231,
    Refcursorarray = 2201,
    Regprocedurearray = 2207,
    Regoperarray = 2208,
    Regoperatorarray = 2209,
    Regclassarray = 2210,
    Regcollationarray = 4192,
    Regtypearray = 2211,
    Regrolearray = 4097,
    Regnamespacearray = 4090,
    Uuidarray = 2951,
    PgLsnarray = 3221,
    Tsvectorarray = 3643,
    Gtsvectorarray = 3644,
    Tsqueryarray = 3645,
    Regconfigarray = 3735,
    Regdictionaryarray = 3770,
    Jsonbarray = 3807,
    Jsonpatharray = 4073,
    TxidSnapshotarray = 2949,
    PgSnapshotarray = 5039,
    Int4rangearray = 3905,
    Numrangearray = 3907,
    Tsrangearray = 3909,
    Tstzrangearray = 3911,
    Daterangearray = 3913,
    Int8rangearray = 3927,
    Cstringarray = 1263,
}
impl BuiltinType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BuiltinType::Missing => "MISSING",
            BuiltinType::Bool => "BOOL",
            BuiltinType::Bytea => "BYTEA",
            BuiltinType::Char => "CHAR",
            BuiltinType::Name => "NAME",
            BuiltinType::Int8 => "INT8",
            BuiltinType::Int2 => "INT2",
            BuiltinType::Int2vector => "INT2VECTOR",
            BuiltinType::Int4 => "INT4",
            BuiltinType::Regproc => "REGPROC",
            BuiltinType::Text => "TEXT",
            BuiltinType::Oid => "OID",
            BuiltinType::Tid => "TID",
            BuiltinType::Xid => "XID",
            BuiltinType::Cid => "CID",
            BuiltinType::Oidvector => "OIDVECTOR",
            BuiltinType::Json => "JSON",
            BuiltinType::Xml => "XML",
            BuiltinType::Xid8 => "XID8",
            BuiltinType::Point => "POINT",
            BuiltinType::Lseg => "LSEG",
            BuiltinType::Path => "PATH",
            BuiltinType::Box => "BOX",
            BuiltinType::Polygon => "POLYGON",
            BuiltinType::Line => "LINE",
            BuiltinType::Float4 => "FLOAT4",
            BuiltinType::Float8 => "FLOAT8",
            BuiltinType::Unknown => "UNKNOWN",
            BuiltinType::Circle => "CIRCLE",
            BuiltinType::Macaddr => "MACADDR",
            BuiltinType::Inet => "INET",
            BuiltinType::Cidr => "CIDR",
            BuiltinType::Macaddr8 => "MACADDR8",
            BuiltinType::Aclitem => "ACLITEM",
            BuiltinType::Bpchar => "BPCHAR",
            BuiltinType::Varchar => "VARCHAR",
            BuiltinType::Date => "DATE",
            BuiltinType::Time => "TIME",
            BuiltinType::Timestamp => "TIMESTAMP",
            BuiltinType::Timestamptz => "TIMESTAMPTZ",
            BuiltinType::Interval => "INTERVAL",
            BuiltinType::Timetz => "TIMETZ",
            BuiltinType::Bit => "BIT",
            BuiltinType::Varbit => "VARBIT",
            BuiltinType::Numeric => "NUMERIC",
            BuiltinType::Refcursor => "REFCURSOR",
            BuiltinType::Regprocedure => "REGPROCEDURE",
            BuiltinType::Regoper => "REGOPER",
            BuiltinType::Regoperator => "REGOPERATOR",
            BuiltinType::Regclass => "REGCLASS",
            BuiltinType::Regcollation => "REGCOLLATION",
            BuiltinType::Regtype => "REGTYPE",
            BuiltinType::Regrole => "REGROLE",
            BuiltinType::Regnamespace => "REGNAMESPACE",
            BuiltinType::Uuid => "UUID",
            BuiltinType::Tsvector => "TSVECTOR",
            BuiltinType::Gtsvector => "GTSVECTOR",
            BuiltinType::Tsquery => "TSQUERY",
            BuiltinType::Regconfig => "REGCONFIG",
            BuiltinType::Regdictionary => "REGDICTIONARY",
            BuiltinType::Jsonb => "JSONB",
            BuiltinType::Jsonpath => "JSONPATH",
            BuiltinType::TxidSnapshot => "TXID_SNAPSHOT",
            BuiltinType::PgSnapshot => "PG_SNAPSHOT",
            BuiltinType::Int4range => "INT4RANGE",
            BuiltinType::Numrange => "NUMRANGE",
            BuiltinType::Tsrange => "TSRANGE",
            BuiltinType::Tstzrange => "TSTZRANGE",
            BuiltinType::Daterange => "DATERANGE",
            BuiltinType::Int8range => "INT8RANGE",
            BuiltinType::Record => "RECORD",
            BuiltinType::Recordarray => "RECORDARRAY",
            BuiltinType::Cstring => "CSTRING",
            BuiltinType::Any => "ANY",
            BuiltinType::Anyarray => "ANYARRAY",
            BuiltinType::Void => "VOID",
            BuiltinType::Trigger => "TRIGGER",
            BuiltinType::LanguageHandler => "LANGUAGE_HANDLER",
            BuiltinType::Internal => "INTERNAL",
            BuiltinType::Anyelement => "ANYELEMENT",
            BuiltinType::Anynonarray => "ANYNONARRAY",
            BuiltinType::Anyenum => "ANYENUM",
            BuiltinType::FdwHandler => "FDW_HANDLER",
            BuiltinType::IndexAmHandler => "INDEX_AM_HANDLER",
            BuiltinType::TsmHandler => "TSM_HANDLER",
            BuiltinType::TableAmHandler => "TABLE_AM_HANDLER",
            BuiltinType::Anyrange => "ANYRANGE",
            BuiltinType::Anycompatible => "ANYCOMPATIBLE",
            BuiltinType::Anycompatiblearray => "ANYCOMPATIBLEARRAY",
            BuiltinType::Anycompatiblenonarray => "ANYCOMPATIBLENONARRAY",
            BuiltinType::Anycompatiblerange => "ANYCOMPATIBLERANGE",
            BuiltinType::Boolarray => "BOOLARRAY",
            BuiltinType::Byteaarray => "BYTEAARRAY",
            BuiltinType::Chararray => "CHARARRAY",
            BuiltinType::Namearray => "NAMEARRAY",
            BuiltinType::Int8array => "INT8ARRAY",
            BuiltinType::Int2array => "INT2ARRAY",
            BuiltinType::Int2vectorarray => "INT2VECTORARRAY",
            BuiltinType::Int4array => "INT4ARRAY",
            BuiltinType::Regprocarray => "REGPROCARRAY",
            BuiltinType::Textarray => "TEXTARRAY",
            BuiltinType::Oidarray => "OIDARRAY",
            BuiltinType::Tidarray => "TIDARRAY",
            BuiltinType::Xidarray => "XIDARRAY",
            BuiltinType::Cidarray => "CIDARRAY",
            BuiltinType::Oidvectorarray => "OIDVECTORARRAY",
            BuiltinType::Jsonarray => "JSONARRAY",
            BuiltinType::Xmlarray => "XMLARRAY",
            BuiltinType::Xid8array => "XID8ARRAY",
            BuiltinType::Pointarray => "POINTARRAY",
            BuiltinType::Lsegarray => "LSEGARRAY",
            BuiltinType::Patharray => "PATHARRAY",
            BuiltinType::Boxarray => "BOXARRAY",
            BuiltinType::Polygonarray => "POLYGONARRAY",
            BuiltinType::Linearray => "LINEARRAY",
            BuiltinType::Float4array => "FLOAT4ARRAY",
            BuiltinType::Float8array => "FLOAT8ARRAY",
            BuiltinType::Circlearray => "CIRCLEARRAY",
            BuiltinType::Moneyarray => "MONEYARRAY",
            BuiltinType::Macaddrarray => "MACADDRARRAY",
            BuiltinType::Inetarray => "INETARRAY",
            BuiltinType::Cidrarray => "CIDRARRAY",
            BuiltinType::Macaddr8array => "MACADDR8ARRAY",
            BuiltinType::Aclitemarray => "ACLITEMARRAY",
            BuiltinType::Bpchararray => "BPCHARARRAY",
            BuiltinType::Varchararray => "VARCHARARRAY",
            BuiltinType::Datearray => "DATEARRAY",
            BuiltinType::Timearray => "TIMEARRAY",
            BuiltinType::Timestamparray => "TIMESTAMPARRAY",
            BuiltinType::Timestamptzarray => "TIMESTAMPTZARRAY",
            BuiltinType::Intervalarray => "INTERVALARRAY",
            BuiltinType::Timetzarray => "TIMETZARRAY",
            BuiltinType::Bitarray => "BITARRAY",
            BuiltinType::Varbitarray => "VARBITARRAY",
            BuiltinType::Numericarray => "NUMERICARRAY",
            BuiltinType::Refcursorarray => "REFCURSORARRAY",
            BuiltinType::Regprocedurearray => "REGPROCEDUREARRAY",
            BuiltinType::Regoperarray => "REGOPERARRAY",
            BuiltinType::Regoperatorarray => "REGOPERATORARRAY",
            BuiltinType::Regclassarray => "REGCLASSARRAY",
            BuiltinType::Regcollationarray => "REGCOLLATIONARRAY",
            BuiltinType::Regtypearray => "REGTYPEARRAY",
            BuiltinType::Regrolearray => "REGROLEARRAY",
            BuiltinType::Regnamespacearray => "REGNAMESPACEARRAY",
            BuiltinType::Uuidarray => "UUIDARRAY",
            BuiltinType::PgLsnarray => "PG_LSNARRAY",
            BuiltinType::Tsvectorarray => "TSVECTORARRAY",
            BuiltinType::Gtsvectorarray => "GTSVECTORARRAY",
            BuiltinType::Tsqueryarray => "TSQUERYARRAY",
            BuiltinType::Regconfigarray => "REGCONFIGARRAY",
            BuiltinType::Regdictionaryarray => "REGDICTIONARYARRAY",
            BuiltinType::Jsonbarray => "JSONBARRAY",
            BuiltinType::Jsonpatharray => "JSONPATHARRAY",
            BuiltinType::TxidSnapshotarray => "TXID_SNAPSHOTARRAY",
            BuiltinType::PgSnapshotarray => "PG_SNAPSHOTARRAY",
            BuiltinType::Int4rangearray => "INT4RANGEARRAY",
            BuiltinType::Numrangearray => "NUMRANGEARRAY",
            BuiltinType::Tsrangearray => "TSRANGEARRAY",
            BuiltinType::Tstzrangearray => "TSTZRANGEARRAY",
            BuiltinType::Daterangearray => "DATERANGEARRAY",
            BuiltinType::Int8rangearray => "INT8RANGEARRAY",
            BuiltinType::Cstringarray => "CSTRINGARRAY",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeType {
    Unknown = 0,
    Insert = 1,
    Update = 2,
    Delete = 3,
}
impl ChangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ChangeType::Unknown => "Unknown",
            ChangeType::Insert => "Insert",
            ChangeType::Update => "Update",
            ChangeType::Delete => "Delete",
        }
    }
}
