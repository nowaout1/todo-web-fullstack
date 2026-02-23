create table todos (
    id uuid primary key default gen_random_uuid(),
    title varchar(100) not null,
    created_at timestamptz not null default current_timestamp
);
