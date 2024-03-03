use std::env;
use std::env::Args;

/*
The `prepare-commit-msg` is executed before we see actual editor that lets us write commit message.
Our binary will receive three arguments: path to the file with initial commit message,
the type of the commit and commit SHA-1.
*/
fn main() -> () {
    for argument in env::args() {
        println!("{argument:?}");
    }

    const TEMPLATE: &'static str = get_template();
    println!("{TEMPLATE}");
}

struct PrepareMessageArgs {
    filename: String,
    commit_type: Option<String>, // TODO replace with an Enum in future
    id: Option<String>
}

fn process_args(args: Args) -> PrepareMessageArgs {
    if args.count() == 0 {
        panic!("Missing prepare-commit-msg arguments");
    }

    return PrepareMessageArgs {
        filename: args.next().unwrap(),
        commit_type: args.next(),
        id: args.next()
    }
}

const fn get_template() -> &'static str {
    return r#"
<type>[optional scope]: <description>

# [optional body]

# [optional footer(s)]
"#;
}