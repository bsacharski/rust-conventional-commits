use std::env;

/*
The `prepare-commit-msg` is executed before we see actual editor that lets us write commit message.
Our binary will receive three arguments: path to the file with initial commit message,
the type of the commit and commit SHA-1.
*/
fn main() -> () {
    for argument in env::args_os() {
        println!("{argument:?}");
    }
}
