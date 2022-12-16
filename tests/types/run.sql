SELECT pg_drop_replication_slot('slot');
SELECT pg_create_logical_replication_slot('slot', 'pboutput');

CREATE TABLE example
(
    a  int,
    b  bigint,
    c  text,
    d  bytea,
    e  float,
    f  double precision,
    aa int[],
    ab bigint[]
);

INSERT INTO example (a, b, c, d, e, f, aa, ab)
VALUES (1, 2, 'a', '\x0011', 0.1, 0.2, '{1}', '{2}');