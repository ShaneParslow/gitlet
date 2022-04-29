mod blob;
mod commit;
mod ops;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter a command.");
        return;
    }

    let op: &str = args[1].as_str();
    if !util::is_repo() && op != "init" {
        println!("Not in an initialized Gitlet directory.");
        return;
    }
    match op {
        "add" => ops::add(),
        "branch" => ops::branch(),
        "init" => ops::init(),
        "status" => ops::status(),
        _ => println!("No command with that name exists."),
    }
    return;
}
