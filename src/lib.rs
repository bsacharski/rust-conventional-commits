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

    struct Header {
        commit_type: CommitType,
        scopes: Option<Vec<String>>,
        description: String,
        has_breaking_change_marker: bool,
    }

    struct Footer {
        elements: Vec<FooterElement>,
        has_breaking_change_marker: bool,
    }

    struct FooterElement {
        content: String,
    }

    #[derive(Debug, PartialEq)]
    pub enum CommitType {
        Fix,
        Feat,
        Custom(String),
    }

    #[derive(Debug)]
    pub struct ParseError {
        pub line: String,
        pub reason: String,
    }
    pub fn parse(commit_message: &str) -> Result<ConventionalCommit, ParseError> {
        // TODO we should split the commit_message using newlines
        if !super::core::SUBJECT_REGEX.is_match(commit_message) {
            return Err(ParseError {
                line: String::from(commit_message),
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

    enum ParserState {
        Init,
        Header,
        Body,
        Footer,
    }

    struct Parser {
        state: ParserState,
        header: Option<Header>,
    }

    impl Parser {
        pub fn new() -> Self {
            Self {
                state: ParserState::Init,
                header: None,
            }
        }

        pub fn process_line(mut self, line: &str, next_line: Option<&str>) -> () {
            match self.state {
                ParserState::Init => {
                    let header = Self::parse_header(line, next_line).unwrap();
                    self.state = ParserState::Header;
                    self.header = Some(header);
                }
                _ => {
                    panic!("Not implemented yet!");
                }
            }
        }

        fn parse_header(line: &str, next_line: Option<&str>) -> Result<Header, ParseError> {
            if !super::core::SUBJECT_REGEX.is_match(line) {
                return Err(ParseError {
                    line: String::from(line),
                    reason: String::from("Commit header has invalid format"),
                });
            }

            if next_line.is_some_and(|l| !l.is_empty()) {
                return Err(ParseError {
                    line: String::from(next_line.unwrap()),
                    reason: String::from("Line that follows header should be empty"),
                });
            }

            let captures = super::core::SUBJECT_REGEX.captures(line).unwrap();
            let commit_type = captures.name("type").unwrap().as_str();
            let scopes = captures.name("scope");
            let description = captures.name("description").unwrap().as_str();
            let has_breaking_change_marker = captures.name("breaking").is_some();

            return Ok(Header {
                commit_type: parse_commit_type(commit_type),
                description: String::from(description),
                scopes: if scopes.is_some() {
                    Some(parse_scopes(scopes.unwrap().as_str()))
                } else {
                    None
                },
                has_breaking_change_marker,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::{parse, CommitType, ConventionalCommit};

    #[test]
    fn should_parse_commit_subject_line_with_feat_type_and_foo_scope() {
        // given
        let subject = String::from("feat(foo): bar baz");

        // when
        let result = parse(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scopes: Some(vec![String::from("foo")]),
            description: String::from("bar baz"),
            body: None,
            footer: None,
            is_breaking_change: false,
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn should_parse_commit_subject_line_with_fix_type_and_foo_scope() {
        // given
        let subject = String::from("fix(foo): bar baz");

        // when
        let result = parse(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: CommitType::Fix,
            scopes: Some(vec![String::from("foo")]),
            description: String::from("bar baz"),
            body: None,
            footer: None,
            is_breaking_change: false,
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn should_parse_commit_subject_line_with_docs_type_and_foo_scope() {
        // given
        let subject = String::from("docs(foo): bar baz");

        // when
        let result = parse(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: CommitType::Custom(String::from("docs")),
            scopes: Some(vec![String::from("foo")]),
            description: String::from("bar baz"),
            body: None,
            footer: None,
            is_breaking_change: false,
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn should_parse_commit_subject_line_with_feat_type_foo_scope_and_breaking_change_marker() {
        // given
        let subject = String::from("feat(foo)!: bar baz");

        // when
        let result = parse(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scopes: Some(vec![String::from("foo")]),
            description: String::from("bar baz"),
            body: None,
            footer: None,
            is_breaking_change: true,
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn should_parse_commit_subject_line_with_fix_type_and_foo_and_bax_scopes() {
        // given
        let subject = String::from("feat(foo,bax): bar baz");

        // when
        let result = parse(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: CommitType::Feat,
            scopes: Some(vec![String::from("foo"), String::from("bax")]),
            description: String::from("bar baz"),
            body: None,
            footer: None,
            is_breaking_change: false,
        };

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn should_return_parse_error_when_subject_line_has_incorrect_syntax() {
        // given
        let subject = String::from("Implemented something");

        // when
        let result = parse(&subject);

        // then
        assert!(result.is_err(), "An Error should have been returned");
    }
}
