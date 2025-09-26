use convert_case::Casing;

pub fn write_file(model_name: String, key: &str, output: String) {
    let dir_path = match option_env!("API_MODEL_ARTIFACT_DIR") {
        Some(dir) => format!("{}/{}", dir, key),
        None => {
            let current_dir = std::env::current_dir().unwrap();
            format!(
                "{}",
                current_dir
                    .join(format!(".build/{}", key))
                    .to_str()
                    .unwrap()
            )
        }
    };

    let file_path = format!(
        "{}/{}.rs",
        dir_path,
        model_name.to_case(convert_case::Case::Snake)
    );

    let dir = std::path::Path::new(&dir_path);

    use std::fs;

    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir) {
            tracing::error!("Failed to create directory: {}", e);
        }
    }

    if let Err(e) = fs::write(&file_path, output.to_string()) {
        tracing::error!("Failed to write file: {}", e);
    } else {
        tracing::info!("generated code {} into {}", model_name, file_path);
    }
}
