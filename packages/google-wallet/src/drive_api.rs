use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

const BASE_URL: &str = "https://www.googleapis.com/drive/v3";

#[derive(Debug, Clone)]
pub struct DriveApi {
    client: Client,
    access_token: String,
}

impl DriveApi {
    /// Creates a new DriveApi instance
    pub fn new(access_token: String) -> Self {
        DriveApi {
            client: Client::new(),
            access_token,
        }
    }

    /// List files in the Google Drive
    pub async fn list_files(&self) -> Result<Vec<File>, Box<dyn Error>> {
        let url = format!("{}/files", BASE_URL);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .query(&[("spaces", "appDataFolder")]) // List files from appDataFolder
            .send()
            .await?;

        if response.status().is_success() {
            let file_list: FileList = response.json().await?;
            Ok(file_list.files)
        } else {
            let error_msg = response.text().await?;
            Err(format!("Error listing files: {}", error_msg).into())
        }
    }

    /// Upload a file to the appDataFolder
    pub async fn upload_file(&self, content: &str) -> Result<File, Box<dyn Error>> {
        tracing::debug!("Uploading file to Google Drive: {content}");
        let metadata = serde_json::json!({
            "name": option_env!("ENV").unwrap_or("local"),
            "parents": ["appDataFolder"]
        });

        let url = format!("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart");

        let form = reqwest::multipart::Form::new()
            .part(
                "metadata",
                reqwest::multipart::Part::text(metadata.to_string())
                    .mime_str("application/json")?, // Correct Content-Type for metadata
            )
            .part(
                "file",
                reqwest::multipart::Part::text(content.to_string()).mime_str("text/plain")?,
            );
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.access_token)
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let uploaded_file: File = response.json().await?;
            Ok(uploaded_file)
        } else {
            let error_msg = response.text().await?;
            Err(format!("Error uploading file: {}", error_msg).into())
        }
    }

    pub async fn get_file(&self, file_id: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/files/{}?alt=media", BASE_URL, file_id);

        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;

        if response.status().is_success() {
            let contents = response.text().await?;
            Ok(contents)
        } else {
            let error_msg = response.text().await?;
            Err(format!("Error getting file: {}", error_msg).into())
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FileList {
    pub files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub id: String,
    pub name: String,
}
