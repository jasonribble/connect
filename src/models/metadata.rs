use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq)]
struct Metadata {
    starred: bool,
    is_archived: bool,
    create_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_seen_at: Option<DateTime<Utc>>,
    next_reminder_at: Option<DateTime<Utc>>,
    frequency: Option<String>,
    last_reminder_at: Option<DateTime<Utc>>,
}
impl Metadata {
    #[allow(dead_code)]
    fn default() -> Self {
        let now = Utc::now();

        Self {
            starred: false,
            is_archived: false,
            create_at: now,
            updated_at: now,
            last_seen_at: None,
            next_reminder_at: None,
            frequency: None,
            last_reminder_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::Metadata;

    #[test]
    fn test_has_default() {
        let default_metadata = Metadata::default();

        let now = Utc::now();

        let expected_metadata = Metadata {
            starred: false,
            is_archived: false,
            create_at: now,
            updated_at: now,
            last_seen_at: None,
            next_reminder_at: None,
            frequency: None,
            last_reminder_at: None,
        };

        assert_eq!(default_metadata.starred, expected_metadata.starred);
        assert_eq!(default_metadata.is_archived, expected_metadata.is_archived);

        let one_second = Duration::seconds(1);
        let created_at_time_difference =
            (default_metadata.create_at - expected_metadata.create_at).abs();

        assert!(created_at_time_difference <= one_second);

        let updated_at_time_difference =
            (default_metadata.updated_at - expected_metadata.updated_at).abs();

        assert!(updated_at_time_difference <= one_second);

        assert!(default_metadata.updated_at <= expected_metadata.updated_at);

        assert_eq!(
            default_metadata.last_seen_at,
            expected_metadata.last_seen_at
        );
        assert_eq!(
            default_metadata.next_reminder_at,
            expected_metadata.next_reminder_at
        );
        assert_eq!(default_metadata.frequency, expected_metadata.frequency);
        assert_eq!(
            default_metadata.last_reminder_at,
            expected_metadata.last_reminder_at
        );
    }
}
