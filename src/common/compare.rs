use std::path::PathBuf;

// A structure used to compare environment files
#[derive(Debug)]
pub struct CompareFileType {
    pub path: PathBuf,
    pub keys: Vec<String>,
}
