-- Add up migration script here
CREATE TABLE IF NOT EXISTS book (
    id varchar(36) NOT NULL,
    title varchar(42) NOT NULL,
    author varchar(42) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NULL,
    PRIMARY KEY (id),
    UNIQUE(title, author)
);
