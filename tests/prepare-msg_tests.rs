#[cfg(test)]
mod con_comm_tests {
    use con_comm::prepare_msg::{process_args, PrepareMessageArgs};
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
}
