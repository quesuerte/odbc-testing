CREATE TABLE databases (
	id	SERIAL	PRIMARY KEY,
	name	VARCHAR(32),
	uri	VARCHAR(32),
	username	VARCHAR(32),
    pass	VARCHAR(32)
);

CREATE TABLE schemas (
	database_id	INTEGER,
	id	SERIAL	PRIMARY KEY,
	name	VARCHAR(32),
    table_id	INTEGER
);

CREATE TABLE tables (
	schema_id	INTEGER,
	id	SERIAL	PRIMARY KEY,
	name	VARCHAR(32)
);

CREATE TABLE columns (
	table_id	INTEGER,
	id	SERIAL	PRIMARY KEY,
	name	VARCHAR(32)
);

CREATE DATABASE test;
\c test
CREATE TABLE test_table (
	hello	VARCHAR(32),
	goodbye	VARCHAR(32)
);