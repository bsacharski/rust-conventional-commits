#[cfg(test)]
mod con_comm_tests {
    use con_comm::prepare_msg::{can_use_template, process_args, PrepareMessageArgs};
    #[test]
    fn should_process_args_with_only_path_to_commit_file() {
        // given
        let args = vec![String::from("DontCare"), String::from(".git/some/file")];

        // when
        let actual = process_args(args);

        // then
        let expected: PrepareMessageArgs = PrepareMessageArgs {
            filename: String::from(".git/some/file"),
            commit_type: None,
            id: None,
        };

        assert_eq!(expected.filename, actual.filename);
        assert_eq!(expected.commit_type, actual.commit_type);
        assert_eq!(expected.id, actual.id);
    }

    #[test]
    fn should_process_complete_set_of_hook_args() {
        // given
        let args = vec![
            String::from("DontCare"),
            String::from(".git/some/file"),
            String::from("merge"),
            String::from("head"),
        ];

        // when
        let actual = process_args(args);

        // then
        let expected: PrepareMessageArgs = PrepareMessageArgs {
            filename: String::from(".git/some/file"),
            commit_type: Some(String::from("merge")),
            id: Some(String::from("head")),
        };

        assert_eq!(expected.filename, actual.filename);
        assert_eq!(expected.commit_type, actual.commit_type);
        assert_eq!(expected.id, actual.id);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_no_args_are_passed_to_process_args() {
        // given
        let args = vec![];

        // when
        process_args(args);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_not_enough_args_are_passed_to_process_args() {
        // given
        let args = vec![String::from("DontCare")];

        // when
        process_args(args);
    }

    #[test]
    fn can_use_template_should_return_true_when_id_is_none() {
        // given
        let args: PrepareMessageArgs = PrepareMessageArgs {
            filename: String::from(".git/some/file"),
            commit_type: None,
            id: None,
        };

        // when
        let actual = can_use_template(&args);

        // then
        assert_eq!(true, actual);
    }

    #[test]
    fn can_use_template_should_return_false_when_id_is_set() {
        // given
        let args: PrepareMessageArgs = PrepareMessageArgs {
            filename: String::from(".git/some/file"),
            commit_type: Some(String::from("merge")),
            id: Some(String::from("head")),
        };

        // when
        let actual = can_use_template(&args);

        // then
        assert_eq!(false, actual);
    }
}
