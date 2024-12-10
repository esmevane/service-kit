use std::path::PathBuf;

use super::{StorageFile, StoragePath};

#[derive(Clone, Debug)]
pub struct StorageCollection {
    pub pool: sqlx::SqlitePool,
}

async fn get_connection(url: impl AsRef<str>) -> crate::Result<sqlx::SqlitePool> {
    let pool = sqlx::sqlite::SqlitePool::connect(url.as_ref()).await?;
    let migration = include_str!("./sql/collection.sql");

    sqlx::query(migration).execute(&pool).await?;

    Ok(pool)
}

impl StorageCollection {
    pub async fn file_index(new_db: PathBuf) -> crate::Result<Self> {
        let pool = get_connection(format!("{}", new_db.display())).await?;
        let collection = StorageCollection { pool };

        collection
            .insert("/example.md".parse()?, "Hello, world!".as_bytes().into())
            .await?;

        Ok(collection)
    }

    pub async fn insert(&self, path: StoragePath, contents: Vec<u8>) -> crate::Result<()> {
        let insert_or_update = r#"
            insert into files
                (name, path, size, contents) 
            values
                ($1, $2, $3, $4)
            on conflict(name, path) do
                update set
                    size = excluded.size, 
                    contents = excluded.contents
            "#;

        path.expect_absolute()?;

        sqlx::query(insert_or_update)
            .bind(&path.file_name()?)
            .bind(&path.parent()?.to_string())
            .bind(contents.len() as i64)
            .bind(contents)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn len(&self) -> crate::Result<usize> {
        let count: (i64,) = sqlx::query_as("select count(*) from files")
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0 as usize)
    }

    pub async fn all(&self) -> crate::Result<Vec<StorageFile>> {
        let files = sqlx::query_as::<_, StorageFile>("select * from files")
            .fetch_all(&self.pool)
            .await?;

        Ok(files)
    }

    pub async fn get(&self, path: StoragePath) -> crate::Result<StorageFile> {
        path.expect_absolute()?;

        let file =
            sqlx::query_as::<_, StorageFile>("select * from files where path = $1 and name = $2")
                .bind(&path.parent()?.to_string())
                .bind(&path.file_name()?)
                .fetch_one(&self.pool)
                .await?;

        Ok(file)
    }

    pub async fn remove(&self, path: StoragePath) -> crate::Result<()> {
        path.expect_absolute()?;

        sqlx::query("delete from files where path = $1 and name = $2")
            .bind(&path.parent()?.to_string())
            .bind(&path.file_name()?)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
