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

    pub fn process_args(mut args: Vec<String>) -> PrepareMessageArgs {
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

# [optional footer(s)]"#;
    }
}
