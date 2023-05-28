CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE apps (
    id uuid default uuid_generate_v4(),
    tenant varchar not null,
    slug varchar not null,
    name varchar not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    deleted_at timestamptz,
    primary key(id),
    unique(tenant, slug)
);