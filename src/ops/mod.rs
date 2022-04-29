use std::fs;
use std::time::UNIX_EPOCH;

use crate::util;
use crate::commit::Commit;

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
    }
    create_dirs();
    let init_commit: Commit = Commit::new_from_time(UNIX_EPOCH, String::from("initial commit"), None);
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
