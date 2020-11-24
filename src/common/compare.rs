use std::path::PathBuf;

// A structure used to compare environment files
#[derive(Debug)]
pub struct CompareFileType {
    pub path: PathBuf,
    pub keys: Vec<String>,
    pub missing: Vec<String>,
}

#[derive(Debug)]
pub struct CompareWarning {
    pub path: PathBuf,
    pub missing_keys: Vec<String>,
}
