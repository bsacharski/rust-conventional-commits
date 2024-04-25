use crate::core::conventional_commit::{CommitType, ConventionalCommit};
use std::cmp::Ordering;

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

impl std::fmt::Display for SemanticVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.metadata {
            Some(metadata) => write!(
                f,
                "{}.{}.{}+{}",
                self.major, self.minor, self.patch, metadata
            ),
            None => write!(f, "{}.{}.{}", self.major, self.minor, self.patch),
        }
    }
}

impl std::cmp::PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl std::cmp::PartialOrd for SemanticVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        if self.major != other.major {
            return self.major < other.major;
        }

        if self.minor != other.minor {
            return self.minor < other.minor;
        }

        if self.patch != other.patch {
            return self.patch < other.patch;
        }

        return false;
    }

    fn le(&self, other: &Self) -> bool {
        if self.major != other.major {
            return self.major <= other.major;
        }

        if self.minor != other.minor {
            return self.minor <= other.minor;
        }

        if self.patch != other.patch {
            return self.patch <= other.patch;
        }

        return true;
    }

    fn gt(&self, other: &Self) -> bool {
        if self.major != other.major {
            return self.major > other.major;
        }

        if self.minor != other.minor {
            return self.minor > other.minor;
        }

        if self.patch != other.patch {
            return self.patch > other.patch;
        }

        return false;
    }

    fn ge(&self, other: &Self) -> bool {
        if self.major != other.major {
            return self.major >= other.major;
        }

        if self.minor != other.minor {
            return self.minor >= other.minor;
        }

        if self.patch != other.patch {
            return self.patch >= other.patch;
        }

        return false;
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

    #[test]
    fn should_convert_semver_without_metadata_to_string() {
        // given
        let version = SemanticVersion::new(1, 2, 3, None);

        // when
        let actual = version.to_string();

        // then
        assert_eq!(String::from("1.2.3"), actual);
    }

    #[test]
    fn should_convert_semver_with_metadata_to_string() {
        // given
        let version = SemanticVersion::new(1, 2, 3, Some(String::from("20240501")));

        // when
        let actual = version.to_string();

        // then
        assert_eq!(String::from("1.2.3+20240501"), actual);
    }

    #[test]
    fn should_mark_same_version_with_different_metadata_as_equal() {}
}
