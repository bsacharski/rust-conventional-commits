pub mod history {
    use crate::core::core::ConventionalCommit;
    use chrono::{DateTime, Utc};

    // TODO implement me!
    struct HistoryEntry {
        commit: ConventionalCommit,
        timestamp: DateTime<Utc>,
    }
}
