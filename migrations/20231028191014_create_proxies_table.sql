-- Add migration script here

CREATE TABLE proxies(
    id UUID PRIMARY KEY,
    slug VARCHAR NOT NULL,
    namespace_id UUID NOT NULL,

    CONSTRAINT fk_namespace_id
        FOREIGN KEY (namespace_id)
            REFERENCES namespaces (id)
            ON DELETE CASCADE,

    CONSTRAINT unique_namespace_slug
        UNIQUE (slug, namespace_id)
)