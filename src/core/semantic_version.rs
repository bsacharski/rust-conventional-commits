use crate::core::conventional_commit::{CommitType, ConventionalCommit};
use std::cmp::Ordering;
use std::fmt::{Formatter, write};

#[derive(Debug)]
struct SemanticVersion {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<PreRelease>,
    metadata: Option<String>,
}

#[derive(Debug)]
struct PreRelease {
    pre_release_type_chain: Vec<PreReleaseType>,
    version: Option<i32>,
}

impl std::fmt::Display for PreRelease {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let type_chain_str = self.pre_release_type_chain
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(".");

        if let Some(version) = self.version {
            write!(f, "{}.{}", type_chain_str, version)
        } else {
            write!(f, "{}", type_chain_str)
        }
    }
}

#[derive(Debug)]
enum PreReleaseType {
    Alpha,
    Beta,
    RC,
}

impl std::fmt::Display for PreReleaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            PreReleaseType::Alpha => "alpha",
            PreReleaseType::Beta => "beta",
            PreReleaseType::RC => "rc",
        };

        write!(f, "{}", str)
    }
}

impl SemanticVersion {
    pub fn new(
        major: u32,
        minor: u32,
        patch: u32,
        pre_release: Option<PreRelease>,
        metadata: Option<String>,
    ) -> Self {
        return Self {
            major,
            minor,
            patch,
            pre_release,
            metadata,
        };
    }

    pub fn from(input: String) -> Option<Self> {
        todo!("Implement parsing string version into proper semver instance")
    }

    pub fn apply_commit(self, commit: ConventionalCommit) -> SemanticVersion {
        if commit.is_breaking_change {
            return Self::new(
                self.major + 1,
                self.minor,
                self.patch,
                self.pre_release,
                self.metadata,
            );
        }

        match commit.commit_type {
            CommitType::Feat => Self::new(
                self.major,
                self.minor + 1,
                self.patch,
                self.pre_release,
                self.metadata,
            ),
            CommitType::Fix => Self::new(
                self.major,
                self.minor,
                self.patch + 1,
                self.pre_release,
                self.metadata,
            ),
            CommitType::Custom(_) => Self::new(
                self.major,
                self.minor,
                self.patch,
                self.pre_release,
                self.metadata,
            ),
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

impl PartialEq for SemanticVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for SemanticVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            return Some(Ordering::Equal);
        }

        return if self.major == other.major {
            if self.minor == other.minor {
                self.patch.partial_cmp(&other.patch)
            } else {
                self.minor.partial_cmp(&other.minor)
            }
        } else {
            self.major.partial_cmp(&other.major)
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::core::conventional_commit::CommitType::{Feat, Fix};
    use crate::core::conventional_commit::{CommitType, ConventionalCommit};
    use crate::core::semantic_version::{PreRelease, SemanticVersion};
    use crate::core::semantic_version::PreReleaseType::{Alpha, Beta, RC};

    #[test]
    fn should_increase_major_version_when_introducing_breaking_change() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None, None);
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
        assert_eq!(new_version, SemanticVersion::new(2, 0, 0, None, None));
    }

    #[test]
    fn should_increase_minor_version_when_introducing_new_feature() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None, None);
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
        assert_eq!(new_version, SemanticVersion::new(1, 1, 0, None, None));
    }

    #[test]
    fn should_increase_minor_version_when_introducing_a_fix() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None, None);
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
        assert_eq!(new_version, SemanticVersion::new(1, 0, 1, None, None));
    }

    #[test]
    fn should_keep_version_as_it_was_when_applying_custom_type_of_commit() {
        // given
        let version = SemanticVersion::new(1, 0, 0, None, None);
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
        assert_eq!(new_version, SemanticVersion::new(1, 0, 0, None, None));
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
                pre_release: None,
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
                pre_release: None,
                metadata: Some(String::from("202105272159"))
            }
        )
    }

    #[test]
    fn should_convert_semver_without_metadata_to_string() {
        // given
        let version = SemanticVersion::new(1, 2, 3, None, None);

        // when
        let actual = version.to_string();

        // then
        assert_eq!(String::from("1.2.3"), actual);
    }

    #[test]
    fn should_convert_semver_with_metadata_to_string() {
        // given
        let version = SemanticVersion::new(1, 2, 3, None, Some(String::from("20240501")));

        // when
        let actual = version.to_string();

        // then
        assert_eq!(String::from("1.2.3+20240501"), actual);
    }

    #[test]
    fn should_mark_same_version_with_different_metadata_as_equal() {
        let first = SemanticVersion::new(1, 2, 3, None, Some(String::from("20240501")));
        let second = SemanticVersion::new(1, 2, 3, None, None);

        assert_eq!(first, second);
    }

    #[test]
    fn should_mark_two_version_with_different_major_as_not_equal() {
        let first = SemanticVersion::new(1, 2, 3, None, None);
        let second = SemanticVersion::new(2, 2, 3, None, None);

        assert_ne!(first, second);
    }

    #[test]
    fn should_mark_two_version_with_different_minor_as_not_equal() {
        let first = SemanticVersion::new(1, 2, 3, None, None);
        let second = SemanticVersion::new(1, 3, 3, None, None);

        assert_ne!(first, second);
    }
    #[test]
    fn should_mark_two_version_with_different_patch_as_not_equal() {
        let first = SemanticVersion::new(1, 2, 3, None, None);
        let second = SemanticVersion::new(1, 2, 4, None, None);

        assert_ne!(first, second);
    }

    #[test]
    fn should_mark_first_version_as_lesser_than_second_patch_change() {
        let first = SemanticVersion::new(1, 2, 3, None, None);
        let second = SemanticVersion::new(1, 2, 4, None, None);

        assert!(first < second)
    }

    #[test]
    fn should_mark_first_version_as_lesser_than_second_minor_change() {
        let first = SemanticVersion::new(1, 2, 3, None, None);
        let second = SemanticVersion::new(1, 3, 3, None, None);

        assert!(first < second)
    }

    #[test]
    fn should_mark_first_version_as_lesser_than_second_major_change() {
        let first = SemanticVersion::new(1, 2, 3, None, None);
        let second = SemanticVersion::new(2, 2, 3, None, None);

        assert!(first < second)
    }

    #[test]
    fn should_mark_first_version_as_greater_than_second_patch_change() {
        let first = SemanticVersion::new(1, 2, 5, None, None);
        let second = SemanticVersion::new(1, 2, 4, None, None);

        assert!(first > second)
    }

    #[test]
    fn should_mark_first_version_as_greater_than_second_minor_change() {
        let first = SemanticVersion::new(1, 3, 4, None, None);
        let second = SemanticVersion::new(1, 2, 4, None, None);

        assert!(first > second)
    }

    #[test]
    fn should_mark_first_version_as_greater_than_second_major_change() {
        let first = SemanticVersion::new(2, 2, 4, None, None);
        let second = SemanticVersion::new(1, 2, 4, None, None);

        assert!(first > second)
    }

    #[test]
    fn should_convert_prerelease_with_alpha_beta_and_version_into_string() {
        let input = PreRelease { pre_release_type_chain: vec![Alpha, Beta], version: Some(1) };

        assert_eq!(input.to_string(), "alpha.beta.1")
    }

    #[test]
    fn should_convert_prerelease_with_alpha_beta_without_versioninto_string() {
        let input = PreRelease { pre_release_type_chain: vec![Alpha, Beta, RC], version: None };

        assert_eq!(input.to_string(), "alpha.beta.rc")
    }
}
