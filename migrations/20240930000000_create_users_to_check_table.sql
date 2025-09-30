CREATE TABLE users_to_check (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    is_checked BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT username_unique UNIQUE (username)
);
