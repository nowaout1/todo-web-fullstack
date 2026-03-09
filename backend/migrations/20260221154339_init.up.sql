create table todos (
    id bigserial primary key,
    title varchar(100) not null,
    created_at timestamptz not null default current_timestamp
);

CREATE INDEX idx_todos_created_at ON todos(created_at DESC);
CREATE INDEX idx_todos_title ON todos(title);

-- Search LIKE '%text%'
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX idx_todos_title_trgm ON todos USING gin (title gin_trgm_ops);
