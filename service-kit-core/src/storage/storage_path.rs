use std::path::PathBuf;

use super::StorageError;

pub struct StoragePath(PathBuf);

impl StoragePath {
    pub fn new(path: PathBuf) -> Self {
        StoragePath::from(path)
    }

    fn into_inner(&self) -> PathBuf {
        self.0.clone()
    }

    pub fn expect_absolute(&self) -> crate::Result<()> {
        if !self.0.is_absolute() {
            return Err(StorageError::BadPath(self.into_inner()))?;
        }

        Ok(())
    }

    pub fn parent(&self) -> Result<StoragePath, StorageError> {
        self.0
            .parent()
            .map(|p| StoragePath(p.to_path_buf()))
            .ok_or(StorageError::MissingPathData)
    }

    pub fn file_name(&self) -> Result<String, StorageError> {
        self.0
            .file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.to_string())
            .ok_or(StorageError::InvalidFileName)
    }
}

impl std::str::FromStr for StoragePath {
    type Err = StorageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StoragePath(PathBuf::from(s)))
    }
}

impl std::fmt::Display for StoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}

impl From<PathBuf> for StoragePath {
    fn from(path: PathBuf) -> Self {
        StoragePath(path)
    }
}
