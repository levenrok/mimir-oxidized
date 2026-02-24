CREATE TABLE scripts(
    id INTEGER PRIMARY KEY,
    name TEXT(1024) NOT NULL UNIQUE,
    content TEXT,
    shebang TEXT(255),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);-->statement breakpoint
CREATE TRIGGER update_timestamp
AFTER UPDATE ON scripts
BEGIN
    UPDATE scripts SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
