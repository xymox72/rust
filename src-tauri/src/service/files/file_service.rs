use async_recursion::async_recursion;
use std::{
    env::{self, VarError},
    io::Error,
    path::{Path, PathBuf},
};
use tokio::fs::{read_dir, remove_dir, remove_file};

use crate::service::log::init_logging;
use log::{info, error};

pub struct FileService {
    working_derictory: String,
}

impl FileService {
    pub fn new() -> Result<Self, VarError> {
        let working_derictory =
            env::var("WORKING_DIRECTORY").expect("WORKING_DIRECTORY isn't presented");
            info!("Initialized FileService with working directory: {}", working_derictory);
        Ok(FileService { working_derictory })
    }

    async fn find_all_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut file_paths = Vec::new();
        self.search_files_recursively(&PathBuf::from(&self.working_derictory), &mut file_paths)
            .await?;
        info!("Found {} all files", file_paths.len());
        Ok(file_paths)
    }

    #[async_recursion]
    async fn search_files_recursively(
        &self,
        dir: &Path,
        file_paths: &mut Vec<PathBuf>,
    ) -> Result<(), Error> {
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

    pub async fn delete_files<F>(&self, files: Vec<String>, emit: F) -> Result<(), Error>
    where
        F: Fn(String) + Send + Sync,
    {
        let files_pathes = self.find_all_files().await?;

        let filted_files: Vec<&PathBuf> = files_pathes
            .iter()
            .filter(|path| {
                if let Some(file_name) = path.file_name() {
                    files.contains(&file_name.to_string_lossy().to_string())
                } else {
                    false
                }
            })
            .collect();
        info!("Found filtred {} files",filted_files.len());

        
        for file_path in filted_files {
            match self.delete_file_by_path(file_path).await {
                Ok(_) => {
                    let format = format!("File '{:?}' deleted successfully", file_path);
                    emit(format);
                    info!("File '{:?}' deleted successfully", file_path);
                },
                Err(e) => {
                    let format = format!("Failed to delete file '{:?}': {:?}", file_path, e);
                    emit(format);
                    error!("Failed to delete file '{:?}': {:?}", file_path, e);
                },
            }
        }
        

        self.delete_empty_directories(&PathBuf::from(&self.working_derictory)).await?;
        

        Ok(())
    }

    async fn delete_file_by_path(&self, file_path: &PathBuf) -> Result<(), Error> {
        let path = file_path;
        remove_file(path).await?;

        Ok(())
    }

    #[async_recursion]
    async fn delete_empty_directories(&self, dir: &Path) -> Result<(), Error> {
        let mut entries = read_dir(dir).await?;
        let mut is_empty = true;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                self.delete_empty_directories(&path).await?;
            }

            if path.exists() {
                is_empty = false;
            }
        }

        if is_empty {
            remove_dir(dir).await?;
            info!("Deleted empty directory: {:?}", dir);
        }

        Ok(())
    }
}
