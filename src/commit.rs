use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct Commit {
    timestamp: SystemTime,
    description: String,
    parent_hash: Option<String>,
    // TODO: Need blobs
}

impl Commit {
    pub fn new_from_time(
        timestamp: SystemTime,
        description: String,
        parent_hash: Option<String>,
    ) -> Self {
        Commit {
            timestamp,
            description,
            parent_hash,
        }
    }

    pub fn new(description: String, parent_hash: Option<String>) -> Self {
        let timestamp: SystemTime = SystemTime::now();
        Commit::new_from_time(timestamp, description, parent_hash)
    }

    pub fn hash(serial: &str) {}
    /*
        fn new_from_file(path: &Path) -> Commit {
            // TODO: Deserialize
        }
    */
    fn serialize(&self) -> String {
        serde_yaml::to_string(self).expect("serialization error")
    }

    fn write_to_path(&self, path: &Path) {
        let mut file = std::fs::File::create(path).expect("Failed to create commit file");
        let serialized = self.serialize();
        file.write_all(serialized.as_bytes())
            .expect("Failed to write to commit file");
    }
}
