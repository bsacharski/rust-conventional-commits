use std::env;
use std::fs::File;
use std::io::Write;

/*
The `prepare-commit-msg` is executed before we see actual editor that lets us write commit message.
Our binary will receive three arguments: path to the file with initial commit message,
the type of the commit and commit SHA-1.
*/
fn main() -> () {
    let args: PrepareMessageArgs = process_args(env::args().collect());

    if !can_use_template(&args) {
        return;
    }

    let mut file = match File::create(&args.filename) {
        Err(e) => panic!("Couldn't open file {}: {}", &args.filename, e),
        Ok(file) => file,
    };

    if let Err(e) = file.write_all(get_template().as_bytes()) {
        panic!("Couldn't write template to file {}: {}", &args.filename, e);
    }

    let _ = file.flush();
}

#[derive(Debug)]
struct PrepareMessageArgs {
    filename: String,
    commit_type: Option<String>, // TODO replace with an Enum in future
    id: Option<String>,
}

fn can_use_template(args: &PrepareMessageArgs) -> bool {
    if args.id.is_none() {
        // we naively expect that if we have a commit ID,
        // then we are amending and already have commit message
        return true;
    }

    return false;
}

fn process_args(mut args: Vec<String>) -> PrepareMessageArgs {
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

const fn get_template() -> &'static str {
    return r#"#<type>[optional scope]: <description>

# [optional body]

# [optional footer(s)]"#;
}

#[cfg(test)]
mod tests {
    use crate::{process_args, PrepareMessageArgs};

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
