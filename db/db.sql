CREATE TABLE IF NOT EXISTS todo
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    checked boolean DEFAULT false
);

CREATE TABLE IF NOT EXISTS account
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
);

CREATE TABLE IF NOT EXISTS expense
(
    id SERIAL PRIMARY KEY NOT NULL,
    account_id integer NOT NULL REFERENCES account (id),
    amount BigInt NOT NULL,
    category VARCHAR(255) NOT NULL,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    merchant VARCHAR(255) NOT NULL,
    notes VARCHAR(255) NOT NULL DEFAULT ''
);

INSERT INTO "public"."account"("name") VALUES('Chase Main') RETURNING "id", "name", "created_at";
INSERT INTO "public"."account"("name") VALUES('Chase Amazon') RETURNING "id", "name", "created_at";
INSERT INTO "public"."account"("name") VALUES('AMEX') RETURNING "id", "name", "created_at";
INSERT INTO "public"."account"("name") VALUES('Chase Business') RETURNING "id", "name", "created_at";

INSERT INTO "public"."expense"("id","account_id","amount","category","created_at","merchant","notes")
VALUES
(24,4,12000,E'456',E'2021-01-01 06:59:10.711821+00',E'456',E''),
(23,4,12000,E'456',E'2021-01-01 06:59:05.290568+00',E'456',E''),
(22,4,12000,E'456',E'2021-01-01 06:58:59.478173+00',E'456',E''),
(21,4,12000,E'456',E'2021-01-01 06:58:53.381708+00',E'456',E''),
(20,4,12000,E'456',E'2021-01-01 06:58:47.188474+00',E'456',E''),
(19,4,500,E'5',E'2021-01-01 06:57:01.621703+00',E'55',E'null'),
(18,2,400,E'4',E'2021-01-01 06:56:27.058793+00',E'4',E'null'),
(17,3,300,E'3',E'2021-01-01 06:56:19.792611+00',E'3',E'null'),
(16,3,200,E'2',E'2021-01-01 06:56:12.05253+00',E'2',E'null'),
(15,4,100,E'1',E'2021-01-01 06:56:02.711988+00',E'1',E'null'),
(14,3,23400,E'234',E'2021-01-01 06:39:51.424681+00',E'234',E'""'),
(13,4,23400,E'234',E'2021-01-01 03:58:01.92791+00',E'234',E'""'),
(12,3,23400,E'234',E'2021-01-01 03:53:37.507937+00',E'234',E'33'),
(11,4,12300,E'123',E'2020-12-29 04:46:12.484122+00',E'123',E'""');