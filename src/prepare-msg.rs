use con_comm::prepare_msg::{can_use_template, get_template, process_args, PrepareMessageArgs};
use std::env;
use std::fs::File;
use std::io::Write;

fn main() -> () {
    /*
    The `prepare-commit-msg` is executed before we see actual editor that lets us write commit message.
    Our binary will receive three arguments: path to the file with initial commit message,
    the type of the commit and commit SHA-1.
    */
    let args: PrepareMessageArgs = process_args(&env::args().collect());

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

#[cfg(test)]
mod tests {
    use con_comm::prepare_msg::{can_use_template, process_args, PrepareMessageArgs};
    #[test]
    fn should_process_args_with_only_path_to_commit_file() {
        // given
        let args = vec![String::from("DontCare"), String::from(".git/some/file")];

        // when
        let actual = process_args(&args);

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
        let actual = process_args(&args);

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
        process_args(&args);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_not_enough_args_are_passed_to_process_args() {
        // given
        let args = vec![String::from("DontCare")];

        // when
        process_args(&args);
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
