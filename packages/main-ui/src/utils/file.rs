#![allow(unused)]
use crate::dioxus_elements::FileEngine;
use crate::services::backend_api::BackendApi;
use dto::{AssetPresignedUrisReadAction, File, FileExtension, FileType};
use std::str::FromStr;
use std::sync::Arc;

fn human_readable_size(bytes: usize) -> String {
    let sizes = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut index = 0;

    while size >= 1024.0 && index < sizes.len() - 1 {
        size /= 1024.0;
        index += 1;
    }

    format!("{:.2} {}", size, sizes[index])
}

#[cfg(feature = "web")]
pub async fn handle_file_upload(file_engine: Arc<dyn FileEngine>, api: BackendApi) -> Vec<File> {
    let mut result: Vec<File> = vec![];
    let files = file_engine.files();

    for f in files {
        match file_engine.read_file(f.as_str()).await {
            Some(bytes) => {
                let file_name: String = f.into();
                let file_name_copy = file_name.clone();
                let ext = file_name.rsplitn(2, '.').nth(0).unwrap_or("").to_string();

                let file_type = FileType::from_str(&ext.clone());
                let file_type = if file_type.is_ok() {
                    Some(file_type.unwrap())
                } else {
                    None
                };

                let req = AssetPresignedUrisReadAction {
                    action: None,
                    total_count: None,
                    file_type,
                };

                let extension = FileExtension::from_str(&ext);

                match extension {
                    Ok(ext) => {
                        let url = match api.upload_metadata(bytes.clone(), req).await {
                            Ok(v) => Some(v),
                            Err(_) => None,
                        };

                        result.push(File {
                            name: file_name,
                            size: human_readable_size(bytes.len()),
                            ext,
                            url,
                        });
                    }
                    Err(_) => {
                        tracing::error!("Not Allowed file extension {}", ext);
                        continue;
                    }
                }
            }
            None => {
                tracing::error!("Error reading file");
                continue;
            }
        };
    }
    result
}
