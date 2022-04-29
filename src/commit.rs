use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::util;

#[derive(Serialize, Deserialize)]
pub struct Commit {
    timestamp: SystemTime,
    description: String,
    parent_hash: Option<String>,
    // TODO: Need blobs
}

impl Commit {
    /// Make a new commit from a description and a parent hash. Timestamp is now.
    pub fn new(description: String, parent_hash: Option<String>) -> Self {
        let timestamp: SystemTime = SystemTime::now();
        Commit::new_from_time(timestamp, description, parent_hash)
    }

    /// Make a new commit with a supplied timestamp.
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

    /// Write commit to .gitlet/commits/*hash*
    /// Serializes this commit and hashes the serialized data
    /// Returns the hash of the commit
    pub fn write(&self) -> String {
        let serialized = self.serialize();
        let hash = self.hash(serialized.as_str());
        let path = PathBuf::from(".gitlet/commits").join(&hash);
        self.write_to_path(path.as_path(), serialized);
        hash
    }

    fn write_to_path(&self, path: &Path, data: String) {
        let mut file = std::fs::File::create(path).expect("Failed to create commit file");
        file.write_all(data.as_bytes())
            .expect("Failed to write to commit file");
    }

    fn hash(&self, data: &str) -> String {
        util::hash_string(data)
    }
    /*
        fn new_from_file(path: &Path) -> Self {
            // TODO: Deserialize
        }
    */
    fn serialize(&self) -> String {
        serde_yaml::to_string(self).expect("serialization error")
    }
}
