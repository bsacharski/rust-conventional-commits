pub mod commit_msg {

    #[derive(Debug)]
    pub struct CommitMsgArgs {
        pub filename: String,
    }

    pub fn process_args(args: &Vec<String>) -> CommitMsgArgs {
        if args.len() < 2 {
            panic!("Missing commit-msg arguments");
        }

        return CommitMsgArgs {
            filename: String::from(args.get(1).unwrap()),
        };
    }
}
