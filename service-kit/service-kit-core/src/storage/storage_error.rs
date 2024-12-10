use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("missing file name")]
    MissingFileName,
    #[error("invalid file name")]
    InvalidFileName,
    #[error("missing path data")]
    MissingPathData,
    #[error("bad path: {0}")]
    BadPath(PathBuf),
}
