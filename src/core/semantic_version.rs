use crate::core::conventional_commit::{CommitType, ConventionalCommit};

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

    pub fn from(input: String) -> Option<Self> {
        todo!("Implement parsing string version into proper semver instance")
    }

    pub fn apply_commit(self, commit: ConventionalCommit) -> SemanticVersion {
        if commit.is_breaking_change {
            return Self::new(self.major + 1, self.minor, self.patch, self.metadata);
        }

        match commit.commit_type {
            CommitType::Feat => Self::new(self.major, self.minor + 1, self.patch, self.metadata),
            CommitType::Fix => Self::new(self.major, self.minor, self.patch + 1, self.metadata),
            CommitType::Custom(_) => Self::new(self.major, self.minor, self.patch, self.metadata),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::conventional_commit::CommitType::{Feat, Fix};
    use crate::core::conventional_commit::{CommitType, ConventionalCommit};
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

    #[test]
    fn should_increase_minor_version_when_introducing_new_feature() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None);
        let commit = ConventionalCommit {
            commit_type: Feat,
            is_breaking_change: false,
            description: String::from("Some new feature"),
            body: None,
            footer: None,
            scopes: None,
        };

        // when
        let new_version = version.apply_commit(commit);

        // then
        assert_eq!(new_version, SemanticVersion::new(1, 1, 0, None));
    }

    #[test]
    fn should_increase_minor_version_when_introducing_a_fix() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None);
        let commit = ConventionalCommit {
            commit_type: Fix,
            is_breaking_change: false,
            description: String::from("Some bugfix"),
            body: None,
            footer: None,
            scopes: None,
        };

        // when
        let new_version = version.apply_commit(commit);

        // then
        assert_eq!(new_version, SemanticVersion::new(1, 0, 1, None));
    }

    #[test]
    fn should_keep_version_as_it_was_when_applying_custom_type_of_commit() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None);
        let commit = ConventionalCommit {
            commit_type: CommitType::Custom(String::from("docs")),
            is_breaking_change: false,
            description: String::from("Updated documentation"),
            body: None,
            footer: None,
            scopes: None,
        };

        // when
        let new_version = version.apply_commit(commit);

        // then
        assert_eq!(new_version, SemanticVersion::new(1, 0, 0, None));
    }

    #[test]
    fn should_parse_semantic_version_string_into_semantic_version_instance_with_metadata() {
        // given
        let input = String::from("32.12.4+202105272159");

        // when
        let actual = SemanticVersion::from(input);

        // then
        assert_eq!(
            actual.unwrap(),
            SemanticVersion {
                major: 32,
                minor: 12,
                patch: 4,
                metadata: Some(String::from("202105272159"))
            }
        )
    }

    #[test]
    fn should_parse_semantic_version_string_with_v_as_prefix_into_semantic_version_instance_with_metadata(
    ) {
        // given
        let input = String::from("v32.12.4+202105272159");

        // when
        let actual = SemanticVersion::from(input);

        // then
        assert_eq!(
            actual.unwrap(),
            SemanticVersion {
                major: 32,
                minor: 12,
                patch: 4,
                metadata: Some(String::from("202105272159"))
            }
        )
    }
}
