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
    use std::collections::VecDeque;
    use std::ops::Deref;

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
        static ref FOOTER_REGEX: Regex = RegexBuilder::new(
            r"^(?:(?<breaking>BREAKING CHANGE)|(?:[-A-z]+)+?)(?::\s)|(?:\s#).+$"
        )
        .build()
        .unwrap();
    }

    #[derive(Debug, PartialEq)]
    pub struct ConventionalCommit {
        pub commit_type: CommitType,
        pub scopes: Option<Vec<String>>,
        pub description: String,
        pub body: Option<Body>,
        pub footer: Option<Footer>,
        pub is_breaking_change: bool,
    }

    impl ConventionalCommit {
        pub fn from(message: CommitMessage) -> Result<Self, ParseError> {
            let mut paragraphs = message.get_paragraphs();
            if paragraphs.len() == 0 {
                return Err(ParseError {
                    line: String::from(""),
                    reason: String::from("Commit message has to have at least one line"),
                });
            }

            let first_paragraph = paragraphs.pop_front().unwrap();
            let potential_header = parse_header(&first_paragraph);
            if potential_header.is_err() {
                return Err(potential_header.err().unwrap());
            }

            let header = potential_header.unwrap();
            let mut body: Option<Body> = None;
            let mut footer: Option<Footer> = None;

            if paragraphs.len() > 0 {
                let last_paragraph = paragraphs.pop_back().unwrap();
                let potential_footer = Footer::from(&last_paragraph);
                if potential_footer.is_ok() {
                    footer = Some(potential_footer.unwrap())
                } else {
                    paragraphs.push_back(last_paragraph)
                }
            }

            if paragraphs.len() > 0 {
                body = Some(Body::from(Vec::from(paragraphs))); // he he, Smash Mouth joke
            }

            return Ok(ConventionalCommit {
                commit_type: header.commit_type,
                scopes: header.scopes,
                description: header.description,
                is_breaking_change: header.has_breaking_change_marker
                    || match &footer {
                        Some(footer) => footer.has_breaking_change_marker,
                        None => false,
                    },
                body,
                footer,
            });
        }
    }

    struct Header {
        commit_type: CommitType,
        scopes: Option<Vec<String>>,
        description: String,
        has_breaking_change_marker: bool,
    }

    #[derive(Debug, PartialEq)]
    pub struct Body {
        pub paragraphs: Vec<Paragraph>,
    }

    impl Body {
        pub fn from(paragraphs: Vec<Paragraph>) -> Self {
            Self { paragraphs }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Footer {
        pub elements: Vec<FooterElement>,
        pub has_breaking_change_marker: bool,
    }

    impl Footer {
        pub fn from(paragraph: &Paragraph) -> Result<Self, ParseError> {
            let mut footer_elements: Vec<FooterElement> = vec![];
            let mut has_breaking_change = false;

            for line in paragraph.get_lines() {
                let potential_element = FooterElement::from(line);
                if potential_element.is_err() {
                    return Err(ParseError {
                        line: String::from(line),
                        reason: String::from("Line does not match git trailer format"),
                    });
                }

                let element = potential_element.unwrap();
                if element.has_breaking_change {
                    has_breaking_change = true;
                }

                footer_elements.push(element);
            }

            return Ok(Self {
                elements: footer_elements,
                has_breaking_change_marker: has_breaking_change,
            });
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct FooterElement {
        pub content: String,
        pub has_breaking_change: bool,
    }

    impl FooterElement {
        pub fn from(line: &String) -> Result<Self, ParseError> {
            let captures = super::core::FOOTER_REGEX.captures(line);
            if captures.is_none() {
                return Err(ParseError {
                    line: String::from(line),
                    reason: String::from("Line does not match git trailer format"),
                });
            }

            let has_breaking_change_marker = captures.unwrap().name("breaking").is_some();

            return Ok(Self {
                content: String::from(line),
                has_breaking_change: has_breaking_change_marker,
            });
        }
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

    pub struct CommitMessage {
        pub paragraphs: Vec<Paragraph>,
    }

    impl CommitMessage {
        pub fn from(file_content: &str) -> Self {
            let mut paragraphs: Vec<Paragraph> = vec![];

            let mut current_paragraph: Paragraph = Paragraph::new();
            for line in file_content.lines() {
                let trimmed_line = line.trim();
                if trimmed_line.len() > 0 {
                    current_paragraph
                        .add_line(trimmed_line)
                        .expect("Failed to add line to paragraph")
                } else {
                    if current_paragraph.len() > 0 {
                        paragraphs.push(current_paragraph);
                    }
                    current_paragraph = Paragraph::new()
                }
            }

            if current_paragraph.len() > 0 {
                paragraphs.push(current_paragraph);
            }

            return CommitMessage { paragraphs };
        }

        pub fn get_paragraphs(&self) -> VecDeque<Paragraph> {
            let mut deque: VecDeque<Paragraph> = VecDeque::with_capacity(self.paragraphs.len());
            for par in self.paragraphs.iter() {
                deque.push_back(Paragraph::from(par));
            }

            return deque;
        }

        pub fn get_paragraph(self: &Self, num: usize) -> Option<&Paragraph> {
            if let ref paragraph = self.paragraphs[num] {
                Some(paragraph)
            } else {
                None
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Paragraph {
        pub lines: Vec<String>,
    }

    impl Paragraph {
        pub fn new() -> Self {
            Self { lines: vec![] }
        }

        pub fn from(other: &Paragraph) -> Self {
            return Self {
                lines: other.lines.to_vec(),
            };
        }

        pub fn add_line(&mut self, line: &str) -> Result<(), ()> {
            if line.len() == 0 {
                return Err(());
            }

            self.lines.push(String::from(line));
            return Ok(());
        }

        pub fn len(&self) -> usize {
            self.lines.len()
        }

        pub fn get_line(&self, num: usize) -> Option<&String> {
            self.lines.get(num)
        }

        pub fn get_lines(&self) -> &Vec<String> {
            &self.lines
        }
    }

    fn parse_header(paragraph: &Paragraph) -> Result<Header, ParseError> {
        if paragraph.len() != 1 {
            return Err(ParseError {
                line: String::from(""),
                reason: String::from("Commit header should have exactly one line"),
            });
        }

        let line = paragraph.get_line(0).unwrap();

        if !super::core::SUBJECT_REGEX.is_match(line) {
            return Err(ParseError {
                line: String::from(line),
                reason: String::from("Commit header has invalid format"),
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

#[cfg(test)]
mod tests {
    use crate::core::CommitType::{Feat, Fix};
    use crate::core::{
        parse, Body, CommitMessage, CommitType, ConventionalCommit, Footer, FooterElement,
        Paragraph,
    };

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

    #[test]
    fn should_create_commit_message_with_separate_three_paragraphs() {
        // given
        let input_string = r"first line of 1st paragraph

first line of 2nd paragraph
second line of 2nd paragraph

first line of 3rd paragraph


first line of 4th paragraph
        ";

        // when
        let commit_message = CommitMessage::from(input_string);

        // then
        assert_eq!(
            commit_message.paragraphs[0].lines,
            vec![String::from("first line of 1st paragraph")]
        );
        assert_eq!(
            commit_message.paragraphs[1].lines,
            vec![
                String::from("first line of 2nd paragraph"),
                String::from("second line of 2nd paragraph")
            ]
        );
        assert_eq!(
            commit_message.paragraphs[2].lines,
            vec![String::from("first line of 3rd paragraph")]
        );
        assert_eq!(
            commit_message.paragraphs[3].lines,
            vec![String::from("first line of 4th paragraph")]
        );
    }

    #[test]
    fn should_create_conventional_commit_with_header_only_variant1() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![Paragraph {
                lines: vec![String::from("feat(unit-test): add new unit tests")],
            }],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Feat,
                scopes: Some(vec![String::from("unit-test")]),
                is_breaking_change: false,
                description: String::from("add new unit tests"),
                body: None,
                footer: None,
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_only_variant2() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![Paragraph {
                lines: vec![String::from("feat(unit-test)!: add new unit tests 2")],
            }],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Feat,
                scopes: Some(vec![String::from("unit-test")]),
                is_breaking_change: true,
                description: String::from("add new unit tests 2"),
                body: None,
                footer: None,
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_only_variant3() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![Paragraph {
                lines: vec![String::from("fix(unit-test,foo): add new unit tests 3")],
            }],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Fix,
                scopes: Some(vec![String::from("unit-test"), String::from("foo")]),
                is_breaking_change: false,
                description: String::from("add new unit tests 3"),
                body: None,
                footer: None,
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_only_variant4() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![Paragraph {
                lines: vec![String::from("fix!: add new unit tests 4")],
            }],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Fix,
                scopes: None,
                is_breaking_change: true,
                description: String::from("add new unit tests 4"),
                body: None,
                footer: None,
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_and_body_variant_1() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![
                Paragraph {
                    lines: vec![String::from("fix!: add new unit tests 5")],
                },
                Paragraph {
                    lines: vec![
                        String::from("Some very interesting line 1"),
                        String::from("Some very interesting line 2"),
                        String::from("Some very interesting line 3"),
                    ],
                },
            ],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Fix,
                scopes: None,
                is_breaking_change: true,
                description: String::from("add new unit tests 5"),
                body: Some(Body {
                    paragraphs: vec![Paragraph {
                        lines: vec![
                            String::from("Some very interesting line 1"),
                            String::from("Some very interesting line 2"),
                            String::from("Some very interesting line 3"),
                        ]
                    }]
                }),
                footer: None,
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_and_body_variant_2() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![
                Paragraph {
                    lines: vec![String::from("fix!: add new unit tests 5")],
                },
                Paragraph {
                    lines: vec![
                        String::from("Some very interesting line 1"),
                        String::from("Some very interesting line 2"),
                        String::from("Some very interesting line 3"),
                    ],
                },
                Paragraph {
                    lines: vec![
                        String::from("Some even more interesting line 1"),
                        String::from("Some even more interesting line 2"),
                        String::from("Some even more interesting line 3"),
                    ],
                },
            ],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Fix,
                scopes: None,
                is_breaking_change: true,
                description: String::from("add new unit tests 5"),
                body: Some(Body {
                    paragraphs: vec![
                        Paragraph {
                            lines: vec![
                                String::from("Some very interesting line 1"),
                                String::from("Some very interesting line 2"),
                                String::from("Some very interesting line 3"),
                            ]
                        },
                        Paragraph {
                            lines: vec![
                                String::from("Some even more interesting line 1"),
                                String::from("Some even more interesting line 2"),
                                String::from("Some even more interesting line 3"),
                            ]
                        }
                    ]
                }),
                footer: None,
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_and_footer_variant_1() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![
                Paragraph {
                    lines: vec![String::from("fix: add new unit tests 5")],
                },
                Paragraph {
                    lines: vec![
                        String::from("Reviewed-by: Foo1234"),
                        String::from("Closes: #5678"),
                    ],
                },
            ],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Fix,
                scopes: None,
                is_breaking_change: false,
                description: String::from("add new unit tests 5"),
                body: None,
                footer: Some(Footer {
                    has_breaking_change_marker: false,
                    elements: vec![
                        FooterElement {
                            content: String::from("Reviewed-by: Foo1234"),
                            has_breaking_change: false,
                        },
                        FooterElement {
                            content: String::from("Closes: #5678"),
                            has_breaking_change: false,
                        }
                    ]
                }),
            }
        )
    }

    #[test]
    fn should_create_conventional_commit_with_header_and_footer_variant_2() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![
                Paragraph {
                    lines: vec![String::from("fix: add new unit tests 5")],
                },
                Paragraph {
                    lines: vec![
                        String::from("BREAKING CHANGE: Foo1234"),
                        String::from("Closes: #5678"),
                    ],
                },
            ],
        };

        // when
        let convention_commit = ConventionalCommit::from(commit);

        // then
        assert_eq!(
            convention_commit.unwrap(),
            ConventionalCommit {
                commit_type: Fix,
                scopes: None,
                is_breaking_change: true,
                description: String::from("add new unit tests 5"),
                body: None,
                footer: Some(Footer {
                    has_breaking_change_marker: true,
                    elements: vec![
                        FooterElement {
                            content: String::from("BREAKING CHANGE: Foo1234"),
                            has_breaking_change: true,
                        },
                        FooterElement {
                            content: String::from("Closes: #5678"),
                            has_breaking_change: false,
                        }
                    ]
                }),
            }
        )
    }
}
