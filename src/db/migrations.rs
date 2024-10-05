use anyhow::{Ok, Result};
use std;

fn get_first_sql_file_path() -> anyhow::Result<String> {
    let mut entries = std::fs::read_dir("migrations")?
        .map(|res| res.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    entries.sort();

    let first_entry = String::from(entries[0].to_string_lossy());

    Ok(first_entry)
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let file_path = get_first_sql_file_path().unwrap();

        println!("{:?}", file_path);

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        assert_eq!(contents, expect_migration);
    }
}
