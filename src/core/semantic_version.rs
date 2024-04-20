use crate::core::conventional_commit::ConventionalCommit;

#[derive(Debug, PartialEq)]
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

    pub fn apply_commit(self, commit: ConventionalCommit) -> SemanticVersion {
        todo!("Implement method for applying commits and thus incrementing version")
    }
}

#[cfg(test)]
mod tests {
    use crate::core::conventional_commit::CommitType::Fix;
    use crate::core::conventional_commit::ConventionalCommit;
    use crate::core::semantic_version::SemanticVersion;

    #[test]
    fn should_increase_major_version_when_introducing_breaking_change() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None);
        let commit = ConventionalCommit {
            commit_type: Fix,
            is_breaking_change: true,
            description: String::from("Some big breaking change"),
            body: None,
            footer: None,
            scopes: None,
        };

        // when
        let new_version = version.apply_commit(commit);

        // then
        assert_eq!(new_version, SemanticVersion::new(2, 0, 0, None));
    }
}
