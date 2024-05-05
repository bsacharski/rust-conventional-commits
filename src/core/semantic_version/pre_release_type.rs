use std::mem::discriminant;

#[derive(Debug)]
enum PreReleaseType {
    Alpha,
    Beta,
    RC,
}

impl PreReleaseType {
    pub fn from(str: &str) -> Option<PreReleaseType> {
        match str {
            "alpha" => Some(PreReleaseType::Alpha),
            "beta" => Some(PreReleaseType::Beta),
            "rc" => Some(PreReleaseType::RC),
            _ => None,
        }
    }
}

impl PartialEq for PreReleaseType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_consider_alpha_as_lesser_than_beta() {
        assert!(Alpha < Beta)
    }

    #[test]
    fn should_consider_beta_as_lesser_than_rc() {
        assert!(Beta < RC)
    }

    #[test]
    fn should_consider_rc_as_greater_than_alpha() {
        assert!(RC > Alpha)
    }

    #[test]
    fn should_convert_alpha_string_to_alpha_enum() {
        assert_eq!(
            PreReleaseType::from("alpha").unwrap(),
            PreReleaseType::Alpha
        )
    }

    #[test]
    fn should_convert_beta_string_to_beta_enum() {
        assert_eq!(PreReleaseType::from("beta").unwrap(), PreReleaseType::Beta)
    }

    #[test]
    fn should_convert_rc_string_to_rc_enum() {
        assert_eq!(PreReleaseType::from("rc").unwrap(), PreReleaseType::RC)
    }

    #[test]
    fn should_return_none_when_trying_to_convert_unknown_value() {
        assert_eq!(PreReleaseType::from("rcc"), None)
    }
}
