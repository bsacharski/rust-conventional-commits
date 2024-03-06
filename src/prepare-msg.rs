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
