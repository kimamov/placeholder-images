CREATE TABLE IF NOT EXISTS image (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    path TEXT NOT NULL,
    source TEXT,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL
    thumbnail TEXT NOT NULL
);