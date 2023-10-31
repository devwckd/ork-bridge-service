-- Add migration script here

CREATE TABLE namespaces (
    id UUID PRIMARY KEY,
    slug VARCHAR NOT NULL,
)