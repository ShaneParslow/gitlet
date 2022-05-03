use crate::commit::Commit;
use std::fs;
use std::time::UNIX_EPOCH;

pub fn create_dirs() {
    fs::create_dir(".gitlet").expect("Could not create gitlet dir");
    fs::create_dir(".gitlet/blobs").expect("Could not create blobs dir");
    fs::create_dir(".gitlet/branches").expect("Could not create branches dir");
    fs::create_dir(".gitlet/commits").expect("Could not create commits dir");
    fs::create_dir(".gitlet/staging").expect("Could not create staging dir");
}

pub fn make_init_commit() -> Commit {
    let description = String::from("initial commit");
    Commit::new_from_time(UNIX_EPOCH, description, None, None)
}
