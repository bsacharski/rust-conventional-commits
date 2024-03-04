use std::env;
use std::env::Args;
use std::fs::File;
use std::io::Write;

/*
The `prepare-commit-msg` is executed before we see actual editor that lets us write commit message.
Our binary will receive three arguments: path to the file with initial commit message,
the type of the commit and commit SHA-1.
*/
fn main() -> () {
    let args: PrepareMessageArgs = get_args();

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

fn get_args() -> PrepareMessageArgs {
    process_args(env::args())
}

fn process_args(mut args: Args) -> PrepareMessageArgs {
    args.next(); // skip binary file name

    if args.len() == 0 {
        panic!("Missing prepare-commit-msg arguments");
    }

    return PrepareMessageArgs {
        filename: args.next().unwrap(),
        commit_type: args.next(),
        id: args.next(),
    };
}

const fn get_template() -> &'static str {
    return r#"#<type>[optional scope]: <description>

# [optional body]

# [optional footer(s)]"#;
}
