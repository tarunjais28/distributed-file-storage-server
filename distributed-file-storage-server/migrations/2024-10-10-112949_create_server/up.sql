CREATE TABLE chunks (
    id UUID PRIMARY KEY,
    file_id UUID NOT NULL,
    chunk_num INT NOT NULL,
    data BYTEA NOT NULL
);

CREATE TABLE files (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    chunk_count INT NOT NULL
);
