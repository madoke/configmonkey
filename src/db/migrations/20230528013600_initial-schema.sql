create extension if not exists "uuid-ossp";

create table domains (
    id uuid default uuid_generate_v4(),
    slug varchar not null,
    created_at timestamptz not null default now(),
    primary key(id),
    unique(slug)
);

create type config_type as enum ('value', 'object', 'array');
create type value_type as enum ('string', 'number', 'boolean');

create table configs (
    id uuid default uuid_generate_v4(),
    domain_id uuid not null,
    key varchar(512) not null,
    type config_type not null,
    version int not null,
    value text,
    parent_id uuid,
    created_at timestamptz not null default now(),
    constraint fk_domains foreign key(domain_id) references domains(id),
    constraint unique_version unique(domain_id, key, version)
);