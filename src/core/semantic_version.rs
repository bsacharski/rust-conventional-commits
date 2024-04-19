use crate::core::conventional_commit::ConventionalCommit;

struct SemanticVersion {
    major: u32,
    minor: u32,
    patch: u32,
    metadata: Option<String>,
}

impl SemanticVersion {
    pub fn new(major: u32, minor: u32, patch: u32, metadata: Option<String>) -> Self {
        return Self {
            major,
            minor,
            patch,
            metadata,
        };
    }

    pub fn apply_commit(commit: ConventionalCommit) -> SemanticVersion {
        todo!("Implement method for applying commits and thus incrementing version")
    }
}
