pub mod prepare_msg {

    #[derive(Debug)]
    pub struct PrepareMessageArgs {
        pub filename: String,
        pub commit_type: Option<String>, // TODO replace with an Enum in future
        pub id: Option<String>,
    }

    pub fn can_use_template(args: &PrepareMessageArgs) -> bool {
        if args.id.is_none() {
            // we naively expect that if we have a commit ID,
            // then we are amending and already have commit message
            return true;
        }

        return false;
    }

    pub fn process_args(args: Vec<String>) -> PrepareMessageArgs {
        if args.len() < 2 {
            panic!("Missing prepare-commit-msg arguments");
        }

        return PrepareMessageArgs {
            filename: String::from(args.get(1).unwrap()),
            commit_type: match args.get(2) {
                Some(commit_type) => Some(String::from(commit_type)),
                None => None,
            },
            id: match args.get(3) {
                Some(id) => Some(String::from(id)),
                None => None,
            },
        };
    }

    pub const fn get_template() -> &'static str {
        return r#"#<type>[optional scope]: <description>

# [optional body]

# [optional footer(s)]

# type can be one of:
#   - fix (correlates with PATCH in semver)
#   - feat (cerrlates with MINOR in semver)
#   - build
#   - chore
#   - ci
#   - docs
#   - style
#   - refactor
#   - perf
#   - test
#     and many, many others
#
# Note: if you add ! after type/scope, or write BREAKING CHANGE
# in the footer, then it is represents a commit that introduces
# some kind of breaking API change (correlates with MAJOR in the
# semver). "#;
    }
}

pub mod commit_msg {

    #[derive(Debug)]
    pub struct CommitMsgArgs {
        pub filename: String,
    }

    pub fn process_args(args: Vec<String>) -> CommitMsgArgs {
        if args.len() < 2 {
            panic!("Missing commit-msg arguments");
        }

        return CommitMsgArgs {
            filename: String::from(args.get(1).unwrap()),
        };
    }
}

pub mod core {
    extern crate lazy_static;
    use lazy_static::lazy_static;
    use regex::{Regex, RegexBuilder};

    lazy_static! {
        static ref SUBJECT_REGEX: Regex = RegexBuilder::new(
            r"
            ^
            (?<type>.+?)
            (:?\((?<scope>.+)\))?
            (?<breaking>!)?
            :
            \s?
            (?<description>.+)
            $
            "
        )
        .case_insensitive(true)
        .ignore_whitespace(true)
        .build()
        .unwrap();
    }

    #[derive(Debug, PartialEq)]
    pub struct ConventionalCommit {
        pub commit_type: CommitType,
        pub scopes: Option<Vec<String>>,
        pub description: String,
        pub body: Option<String>,
        pub footer: Option<Vec<String>>,
        pub is_breaking_change: bool,
    }

    #[derive(Debug, PartialEq)]
    pub enum CommitType {
        Fix,
        Feat,
        Custom(String),
    }

    #[derive(Debug)]
    pub struct ParseError {
        pub header: String,
        pub reason: String,
    }
    pub fn parse(commit_message: &str) -> Result<ConventionalCommit, ParseError> {
        // TODO we should split the commit_message using newlines
        if !super::core::SUBJECT_REGEX.is_match(commit_message) {
            return Err(ParseError {
                header: String::from(commit_message),
                reason: String::from("Commit header has invalid format"),
            });
        }

        let captures = super::core::SUBJECT_REGEX.captures(commit_message).unwrap();
        let commit_type = captures.name("type").unwrap().as_str();
        let scopes = captures.name("scope");
        let description = captures.name("description").unwrap().as_str();
        let has_breaking_change_marker = captures.name("breaking").is_some();

        return Ok(ConventionalCommit {
            commit_type: parse_commit_type(commit_type),
            body: None,
            description: String::from(description),
            footer: None,
            is_breaking_change: has_breaking_change_marker, // todo should look for BREAKING CHANGE footer
            scopes: if scopes.is_some() {
                Some(parse_scopes(scopes.unwrap().as_str()))
            } else {
                None
            },
        });
    }

    fn parse_commit_type(commit_type: &str) -> CommitType {
        return match commit_type.to_lowercase().as_str() {
            "feat" => CommitType::Feat,
            "fix" => CommitType::Fix,
            _ => CommitType::Custom(String::from(commit_type)),
        };
    }

    fn parse_scopes(scopes: &str) -> Vec<String> {
        scopes.split(",").map(|s| String::from(s)).collect()
    }
}
