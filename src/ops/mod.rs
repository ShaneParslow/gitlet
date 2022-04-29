use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

use crate::commit::Commit;
use crate::util;

/* Guaranteed to be in a working directory with a .gitlet dir, with the exception of init */

pub fn add() {
    return;
}

pub fn branch() {
    return;
}

/// Tries to make a new repository in the current working directory
/// Fails if a repository already exists
pub fn init() {
    if util::is_repo() {
        println!("A Gitlet version-control system already exists in the current directory.");
        return;
    }
    create_dirs();
    let init_commit: Commit =
        Commit::new_from_time(UNIX_EPOCH, String::from("initial commit"), None);
    //init_commit.serialize();
    let second_commit = Commit::new(String::from("second commit lol "), None);
    //second_commit.serialize();
}

fn create_dirs() {
    fs::create_dir(".gitlet").expect("Could not create gitlet dir");
    fs::create_dir(".gitlet/blobs").expect("Could not create blobs dir");
    fs::create_dir(".gitlet/branches").expect("Could not create branches dir");
    fs::create_dir(".gitlet/commits").expect("Could not create commits dir");
}

pub fn status() {
    println!("actually really fr got status");
}
