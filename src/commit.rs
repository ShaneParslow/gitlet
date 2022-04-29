use std::path::Path;
use std::time::SystemTime;

pub struct Commit {
    timestamp: SystemTime,
    description: String, // Does this need to be &str for any reason?
    parent: Option<Box<Commit>>,
    // TODO: Need blobs
}

impl Commit {
    pub fn new_from_time(timestamp: SystemTime, description: String, parent: Option<Box<Commit>>) -> Commit {
        Commit{ timestamp, description, parent }
    }

    pub fn new(description: String, parent: Option<Box<Commit>>) -> Commit {
        let timestamp: SystemTime = SystemTime::now();
        Commit::new_from_time(timestamp, description, parent)
    }
/*
    fn new_from_file(path: &Path) -> Commit {
        // TODO: Deserialize
    }

    fn write_to_disk() {
        // ?
    }
 */
}
