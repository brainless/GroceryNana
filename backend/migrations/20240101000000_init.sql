-- Initial migration for GroceryNana database
-- This is a placeholder migration to ensure the database is created

CREATE TABLE IF NOT EXISTS _migration_test (
    id INTEGER PRIMARY KEY,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Insert a test record to verify the database is working
INSERT INTO _migration_test (id) VALUES (1);