mod ops;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough args!");
        return;
    }
    match args[1].as_str() {
        "status" => ops::status(),
        _ => println!("Unknown option"),
    }
    return;
}
