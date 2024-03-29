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
    todo!("Implement log parsing routine");
}
