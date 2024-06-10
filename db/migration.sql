CREATE TABLE IF NOT EXISTS tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    bucket_scope TEXT NOT NULL,
    access TEXT NOT NULL, -- read / write / full
    token TEXT NOT NULL
)