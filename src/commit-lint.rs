use std::env;

/*
 When a `commit-msg` hook is called, the OS will execute a binary with a path to the temporary
 file passed as a first argument. This file contains a commit message that we want our linter
 to run against.
 */
fn main() -> () {
    for argument in env::args_os() {
        println!("{argument:?}");
    }
}