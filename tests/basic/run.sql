SELECT pg_drop_replication_slot('slot');
SELECT pg_create_logical_replication_slot('slot', 'pboutput');

CREATE TABLE IF NOT EXISTS a
(
    a integer
);

INSERT INTO a(a)
values (0);