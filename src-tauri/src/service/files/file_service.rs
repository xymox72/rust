use async_recursion::async_recursion;
use std::{
    env::{self, VarError},
    io::Error,
    path::{Path, PathBuf},
};
use tokio::fs::{read_dir, remove_dir, remove_file};

pub struct FileService {
    working_derictory: String,
}

impl FileService {
    pub fn new() -> Result<Self, VarError> {
        let working_derictory =
            env::var("WORKING_DIRECTORY").expect("WORKING_DIRECTORY isn't presented");
        Ok(FileService { working_derictory })
    }

    pub async fn find_all_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut file_paths = Vec::new();
        self.search_files_recursively(&PathBuf::from(&self.working_derictory), &mut file_paths).await?;
        Ok(file_paths)
    }

    #[async_recursion]
    async fn search_files_recursively(&self, dir: &Path, file_paths: &mut Vec<PathBuf>) -> Result<(), Error> {
        let mut entries = read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                self.search_files_recursively(&path, file_paths).await?;
            } else {
                file_paths.push(path);
            }
        }
        Ok(())
    }



    pub async fn delete_files<F>(&self, files: Vec<String>,  emit: F) -> Result<(), Error>
    where
        F: Fn(String) + Send + Sync, {
        let files_pathes = self.find_all_files().await?;
      
      

   
        /**
        for file_name in files {
           match self.search_and_delete_file(&PathBuf::from(&self.working_derictory), &file_name).await {
               Ok(_) => {
                emit(format!("File '{}' deleted successfully", file_name))
               },
               Err(e) => emit( format!("Failed to delete file '{}': {:?}", file_name, e))
           } 

        }
*/
        Ok(())
    }
    #[async_recursion]
    async fn search_and_delete_file(&self, dir: &Path, file_name: &str) -> Result<(), Error> {
        if dir.is_dir() {
            let mut entries = read_dir(dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.is_dir() {
                    self.search_and_delete_file(&path, file_name).await?;
                } else {


                    if let Some(name) = path.file_name() {
                      
                        let name_str = name.to_string_lossy().to_string();
                        
                     
                        if name_str == file_name {
                            remove_file(&path).await?;      
                        }
                    }
                }
            }
            
            self.delete_empty_directories(dir).await?;
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
