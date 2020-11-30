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