use async_recursion::async_recursion;
use std::{env::{self, VarError}, io::Error, path::Path};
use tokio::fs::{read_dir, remove_file, remove_dir};

pub struct FileService {
    working_derictory: String,
}

impl FileService {
    pub fn new() -> Result<Self, VarError> {
        let working_derictory =
            env::var("WORKING_DIRECTORY").expect("WORKING_DIRECTORY isn't presented");
        Ok(FileService { working_derictory })
    }

    pub async fn delete_files(&self, files: Vec<String>) -> Result<(), Error> {
        for file in files {
            let file_path = self.working_derictory.clone() + "/" + &file;
            if Path::new(&file_path).exists() {
                remove_file(&file_path).await?;
                
                
                // Check if the parent directory is empty after deleting the file
                let parent_dir = Path::new(&file_path).parent().unwrap();
                self.delete_empty_directories(parent_dir).await?;
            } else {
              
            }
        }

        Ok(())
    }

    #[async_recursion]
    async fn delete_empty_directories(&self, dir: &Path) -> Result<(), Error> {
        if read_dir(dir).await?.next_entry().await.is_ok() {
            remove_dir(dir).await?;
           

            // Рекурсивно проверяем родительскую директорию
            if let Some(parent_dir) = dir.parent() {
                self.delete_empty_directories(parent_dir).await?;
            }
        }
        Ok(())
    }
}
