SELECT pg_drop_replication_slot('slot');
SELECT pg_create_logical_replication_slot('slot', 'pboutput');

CREATE TABLE IF NOT EXISTS a
(
    a integer PRIMARY KEY,
    b float
);

ALTER TABLE a
    REPLICA IDENTITY FULL;

INSERT INTO a(a, b)
values (0, 1.0);

DELETE
FROM a
where a = 0;

ALTER TABLE A
    add column c double precision;


INSERT INTO a(a, b, c)
values (0, 1.0, 2.0);