create extension if not exists "uuid-ossp";

create table domains (
    id uuid default uuid_generate_v4() primary key,
    slug varchar not null,
    created_at timestamptz not null default now(),
    constraint domains_unique_slug unique(slug)
);

create type value_type as enum ('string', 'number', 'boolean');

create table configs (
    id uuid default uuid_generate_v4() primary key,
    domain_id uuid not null,
    key varchar(512) not null,
    created_at timestamptz not null default now(),
    constraint configs_fk_domains foreign key(domain_id) references domains(id),
    constraint configs_unique_key unique(domain_id, key)
);

create table values (
    id uuid default uuid_generate_v4() primary key,
    config_id uuid not null,
    type value_type not null,
    version int not null,
    value text not null,
    created_at timestamptz not null default now(),
    constraint values_fk_configs foreign key(config_id) references configs(id) on delete cascade,
    constraint values_unique_version unique(config_id, version)
)