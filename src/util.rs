use sha1::{Digest, Sha1};
use std::path::Path;

/// Checks if the working directory is a gitlet repository
pub fn is_repo() -> bool {
    let repo = Path::new("./.gitlet");
    repo.is_dir()
}

/// Return sha-1 hash of input &str in the form of a String
pub fn hash_string(data: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let hash = hasher.finalize();
    base16ct::lower::encode_string(&hash)
}
