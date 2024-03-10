#[cfg(test)]
mod con_comm_tests {
    use con_comm::core::{parse, CommitType, ConventionalCommit};

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
