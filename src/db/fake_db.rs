#[cfg(test)]
mod test_helpers {
    use anyhow::{Ok, Result};
    use std;
    pub fn first_sql_snapshot() -> anyhow::Result<String> {
        let mut entries = std::fs::read_dir("migrations")?
            .map(|res| res.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        entries.sort();

        let first_entry = String::from(entries[0].to_string_lossy());

        Ok(first_entry)
    }

    pub fn last_sql_snapshot() -> anyhow::Result<String> {
        let mut entries = std::fs::read_dir("migrations")?
            .map(|res| res.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        entries.sort();

        let last_entry = &entries[entries.len() - 1];
        let first_entry = String::from(last_entry.to_string_lossy());

        Ok(first_entry)
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use std::fs;

    #[test]
    fn read_first_sql_file_in_migrations() {
        let expect_migration = r#"-- Add migration script here
CREATE TABLE IF NOT EXISTS contacts
(
id INTEGER PRIMARY KEY AUTOINCREMENT,
first_name TEXT NOT NULL,
last_name TEXT NOT NULL,
display_name TEXT NOT NULL,
email TEXT NOT NULL,
phone_number TEXT NOT NULL
);"#;

        let file_path = first_sql_snapshot().unwrap();

        println!("{:?}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        assert_eq!(contents, expect_migration);
    }

    #[test]
    fn read_last_sql_file_in_migrations() {
        let expect_migration = r#"-- Add migration script here
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
PRAGMA foreign_keys = ON;"#;

        let file_path = last_sql_snapshot().unwrap();

        println!("{:?}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        assert_eq!(contents, expect_migration);
    }
}
