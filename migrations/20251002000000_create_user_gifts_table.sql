CREATE TABLE user_gifts (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
    gifts TEXT[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
