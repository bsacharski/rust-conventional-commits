use con_comm::commit_msg::{process_args, CommitMsgArgs};
use std::fs::File;
use std::io::Read;
use std::{env, fs};

/*
When a `commit-msg` hook is called, the OS will execute a binary with a path to the temporary
file passed as a first argument. This file contains a commit message that we want our linter
to run against.
*/
fn main() -> () {
    let args: CommitMsgArgs = process_args(env::args().collect());

    let file_content = fs::read_to_string(String::from(args.filename))
        .unwrap_or_else(|e| panic!("Couldn't open file with commit message: {}", e));
}
