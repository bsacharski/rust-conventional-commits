use std::process::Command;

fn main() -> () {
    /*
    This binary should:
    1. Parse git log into a history of git commits for given branch
    2. Generate a changelog (doesn't have to be pretty markdown just yet)
    3. Store a reference to last read commit to speed up changelog generation in future

    What we would probably like to do first is to decide on git log format, that is consistent
    among various languages and can be easily parsed.

    git log should provide:
    - timestamp of commit
    - full commit message, in a same format as every other thing parsed so far

    LANG="en_US.UTF-8" git log --format=%ct%n%B should do the trick for now

    LANG sets the env variable for git to use
    %ct adds committer date in unix timestamp
    %n adds newline
    %B adds raw, unprocessed git commit message

    TIL: git provides built-in support for parsing trailers with git interpret-trailers.
    It might be worth looking into that.
     */

    run_git();

    todo!("Implement log parsing routine");
}

fn run_git() -> () {
    // TODO need to fina a good way to provide reliable path to git binary
    let command = Command::new("/usr/bin/git")
        .args(["log", "--format=%ct%n%B"])
        .env("LANG", "en_US.UTF-8")
        .output();

    if command.is_err() {
        panic!(
            "Failed to start git log command: {}",
            command.err().unwrap()
        )
    }

    let stdout_u8 = command.unwrap().stdout;
    let stdout = std::str::from_utf8(stdout_u8.as_slice()).unwrap();
    println!("{}", stdout);
}
