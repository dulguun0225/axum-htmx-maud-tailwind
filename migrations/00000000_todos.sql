CREATE TABLE todos (
    id serial PRIMARY KEY,
    title text NOT NULL,
    CHECK (title <> ''),
    done boolean NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ
);
