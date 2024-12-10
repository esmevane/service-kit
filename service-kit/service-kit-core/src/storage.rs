mod storage_collection;
mod storage_error;
mod storage_file;
mod storage_path;

pub use storage_collection::StorageCollection;
pub use storage_error::StorageError;
pub use storage_file::StorageFile;
pub use storage_path::StoragePath;

#[cfg(test)]
mod test {
    use tempfile::tempdir;

    #[tokio::test]
    async fn file_collections() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let new_db = temp_dir.path().join("new.db");

        // touch the file to make sure it exists
        std::fs::File::create(&new_db).expect("Failed to create temp db file");

        let collection = super::StorageCollection::file_index(new_db).await?;

        collection
            .insert("/base/hello.md".parse()?, "Hello, world!".as_bytes().into())
            .await?;

        let file = collection.get("/base/hello.md".parse()?).await?;

        assert_eq!(file.name, "hello.md");
        assert_eq!(file.size, 13);
        assert_eq!(file.contents, "Hello, world!".as_bytes());

        collection.remove("/base/hello.md".parse()?).await?;

        let file = collection.get("/base/hello.md".parse()?).await;

        assert!(file.is_err());

        Ok(())
    }
}
