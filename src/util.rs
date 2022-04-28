use std::path;

/* Checks if the working directory is a gitlet repository */
pub fn is_repo() -> bool {
    let repo = path::Path::new("./.gitlet");
    repo.is_dir()
}
