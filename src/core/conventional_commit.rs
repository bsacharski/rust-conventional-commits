extern crate lazy_static;

use crate::core::base::{Paragraph, ParseError};
use crate::core::commit_message::CommitMessage;
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
    static ref FOOTER_REGEX: Regex =
        RegexBuilder::new(r"^(?:(?<breaking>BREAKING CHANGE)|(?:[-A-z]+)+?)(?::\s)|(?:\s#).+$")
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
    pub fn from_str(message: &str) -> Result<Self, ParseError> {
        let commit = CommitMessage::from(message);
        return ConventionalCommit::from(commit);
    }

    pub fn from(message: CommitMessage) -> Result<Self, ParseError> {
        let mut paragraphs = message.get_paragraphs();
        if paragraphs.len() == 0 {
            return Err(ParseError {
                line: String::from(""),
                reason: String::from("Commit message has to have at least one line"),
            });
        }

        let first_paragraph = paragraphs.pop_front().unwrap();
        let potential_header = Header::from(&first_paragraph);
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

impl Header {
    pub fn from(paragraph: &Paragraph) -> Result<Header, ParseError> {
        if paragraph.len() != 1 {
            return Err(ParseError {
                line: String::from(""),
                reason: String::from("Commit header should have exactly one line"),
            });
        }

        let line = paragraph.get_line(0).unwrap();

        if !SUBJECT_REGEX.is_match(line) {
            return Err(ParseError {
                line: String::from(line),
                reason: String::from("Commit header has invalid format"),
            });
        }

        let captures = SUBJECT_REGEX.captures(line).unwrap();
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

        /*
         TODO this mechanism will not work! Git trailers may contain whitespace characters, see:
        When reading trailers, there can be no whitespace before or inside the <key>, but any
        number of regular space and tab characters are allowed between the <key> and the separator.
        There can be whitespaces before, inside or after the <value>.
        The <value> may be split over multiple lines with each subsequent line starting with
        at least one whitespace, like the "folding" in RFC 822.

        It looks like we have to go back to the state machine-based approach - maybe we could
        read message using bottom-up approach, when we read a line and if it starts with a space,
        then we load it into a buffer, and keep reading until we hopefully reach the "key". If
        we find an empty newline, or we don't find a line with something that matches "key" pattern,
        then it looks like the buffer contains normal body message. If a "key" is found, then
        we grab the contents of the buffer, trim the unnecessary whitespace characters and join
        the contents of the trailer.
         */

        for line in paragraph.get_lines() {
            let potential_element = FooterElement::from(line.as_str());
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
    pub fn from(line: &str) -> Result<Self, ParseError> {
        let captures = FOOTER_REGEX.captures(line);
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

#[cfg(test)]
mod tests {
    use crate::core::base::Paragraph;
    use crate::core::commit_message::CommitMessage;
    use crate::core::conventional_commit::CommitType::{Feat, Fix};
    use crate::core::conventional_commit::{
        Body, CommitType, ConventionalCommit, Footer, FooterElement,
    };

    #[test]
    fn should_parse_commit_subject_line_with_feat_type_and_foo_scope() {
        // given
        let subject = "feat(foo): bar baz";

        // when
        let result = ConventionalCommit::from_str(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: Feat,
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
        let subject = "fix(foo): bar baz";

        // when
        let result = ConventionalCommit::from_str(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: Fix,
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
        let subject = "docs(foo): bar baz";

        // when
        let result = ConventionalCommit::from_str(&subject);

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
        let subject = "feat(foo)!: bar baz";

        // when
        let result = ConventionalCommit::from_str(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: Feat,
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
        let subject = "feat(foo,bax): bar baz";

        // when
        let result = ConventionalCommit::from_str(&subject);

        // then
        let expected: ConventionalCommit = ConventionalCommit {
            commit_type: Feat,
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
        let subject = "Implemented something";

        // when
        let result = ConventionalCommit::from_str(&subject);

        // then
        assert!(result.is_err(), "An Error should have been returned");
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

    #[test]
    fn should_create_conventional_commit_with_header_and_footer_variant_3() {
        // given
        let commit = CommitMessage {
            paragraphs: vec![
                Paragraph {
                    lines: vec![String::from("fix: add new unit tests 5")],
                },
                // Note git trailers/footer will be always in the last paragraph of commit, so it makes
                // it easier to parse.
                Paragraph {
                    lines: vec![
                        String::from("keyA: This is one-line of git trailer."),
                        String::from("keyB: This is a very long value, with spaces and"),
                        String::from("  newlines in it."),
                        String::from("keyC: This is yet another one-line of git trailer"),
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
                            content: String::from("keyA: This is one-line of git trailer."),
                            has_breaking_change: false,
                        },
                        FooterElement {
                            content: String::from(
                                "keyB: This is a very long value, with spaces and newlines in it."
                            ),
                            has_breaking_change: false,
                        },
                        FooterElement {
                            content: String::from(
                                "keyC: This is yet another one-line of git trailer"
                            ),
                            has_breaking_change: false,
                        },
                    ]
                }),
            }
        )
    }
}
