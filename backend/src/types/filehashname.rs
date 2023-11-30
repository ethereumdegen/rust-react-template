use actix_multipart::form::tempfile::TempFile;

use sha2::{Digest, Sha256};




use std::io::Read;


use ethers::utils::hex;

use futures::stream::StreamExt;
use tokio::{fs, io::AsyncReadExt as _};

#[derive(Clone)]
pub struct FileHashName(String);

#[derive(thiserror::Error, Debug)]
pub enum FileHashNameError {
    #[error("Error reading from temporary file")]
    TempFileReadError,
}

impl FileHashName {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        FileHashName(hex::encode(result))
    }

    pub async fn from_temp_file(temp_file: &TempFile) -> Result<Self, FileHashNameError> {
        let file_path = temp_file
            .file
            .path()
            .to_str()
            .ok_or(FileHashNameError::TempFileReadError)?;

        let mut file = fs::File::open(file_path).await.unwrap();

        let size_estimate = file
            .metadata()
            .await
            .map(|md| md.len())
            .unwrap_or(1024)
            .try_into()
            .expect("file too big");

        let mut contents = Vec::with_capacity(size_estimate);
        file.read_to_end(&mut contents).await.unwrap();

        Ok(FileHashName::from_bytes(&contents))
    }
}

impl From<FileHashName> for String {
    fn from(item: FileHashName) -> String {
        item.0
    }
}
