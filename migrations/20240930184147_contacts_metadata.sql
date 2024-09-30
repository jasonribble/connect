-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts_metadata 
(
contact_id INTEGER NOT NULL,
starred BOOLEAN NOT NULL,
is_archived BOOLEAN NOT NULL,
created_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
last_seen_at TEXT,
next_reminder_at TEXT,
frequency INTEGER,
last_reminder_at TEXT
)