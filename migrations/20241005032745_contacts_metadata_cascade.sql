-- Add migration script here
-- Enable foreign key support
PRAGMA foreign_keys = OFF;

-- Create a temporary table for contacts_metadata
CREATE TABLE contacts_metadata_temp (
    contact_id INTEGER PRIMARY KEY,
    starred BOOLEAN NOT NULL,
    is_archived BOOLEAN NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_seen_at TEXT,
    next_reminder_at TEXT,
    frequency INTEGER,
    last_reminder_at TEXT,
    FOREIGN KEY (contact_id) REFERENCES contacts(id) ON DELETE CASCADE
);

-- Copy data from the old table to the new one
INSERT INTO contacts_metadata_temp SELECT * FROM contacts_metadata;

-- Drop the old table
DROP TABLE contacts_metadata;

-- Rename the new table to the original name
ALTER TABLE contacts_metadata_temp RENAME TO contacts_metadata;

-- Re-enable foreign key support
PRAGMA foreign_keys = ON;