mod init;

use crate::util;

pub fn add() {
    return;
}

pub fn branch() {
    return;
}

/// Tries to make a new repository in the current working directory.
/// Fails if a repository already exists. Makes required directories and an initial commit.
pub fn init() {
    if util::is_repo() {
        println!("A Gitlet version-control system already exists in the current directory.");
        return;
    }
    init::create_dirs();
    let init_commit = init::make_init_commit();
    init_commit.write();

    // TODO: Master branch
    // TODO: Empty staging area
}

pub fn status() {
    println!("actually really fr got status");
}
