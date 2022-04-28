mod ops;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough args!");
        return;
    }

    let op: &str = args[1].as_str();
    if !util::is_repo() && op != "init" {
        println!("Unititialized repository!");
        return;
    }
    match args[1].as_str() {
        "add" => ops::add(),
        "branch" => ops::branch(),
        "init" => ops::init(),
        "status" => ops::status(),
        _ => println!("Unknown option"),
    }
    return;
}
