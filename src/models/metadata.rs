use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Metadata {
    pub contact_id: i64,
    pub starred: bool,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub next_reminder_at: Option<DateTime<Utc>>,
    pub frequency: Option<String>,
    pub last_reminder_at: Option<DateTime<Utc>>,
}

impl Metadata {
    #[allow(dead_code)]
    pub fn new(contact_id: i64) -> Self {
        let now = Utc::now();

        Self {
            contact_id,
            starred: false,
            is_archived: false,
            created_at: now,
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
        let default_metadata = Metadata::new(1);

        let now = Utc::now();

        let expected_metadata = Metadata {
            contact_id: 1,
            starred: false,
            is_archived: false,
            created_at: now,
            updated_at: now,
            last_seen_at: None,
            next_reminder_at: None,
            frequency: None,
            last_reminder_at: None,
        };

        assert_eq!(default_metadata.contact_id, expected_metadata.contact_id);

        assert_eq!(default_metadata.starred, expected_metadata.starred);
        assert_eq!(default_metadata.is_archived, expected_metadata.is_archived);

        let half_second = Duration::milliseconds(500);
        let created_at_time_difference = expected_metadata.created_at - default_metadata.created_at;

        assert!(created_at_time_difference >= Duration::zero());
        assert!(created_at_time_difference < half_second);

        let updated_at_time_difference = expected_metadata.updated_at - default_metadata.updated_at;

        assert!(created_at_time_difference >= Duration::zero());
        assert!(updated_at_time_difference < half_second);

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
