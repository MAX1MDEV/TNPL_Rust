use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn organize_files_by_extension(directory: &str) -> std::io::Result<()> {
    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let folder_name = extension.to_string_lossy().to_string();
                let target_dir = Path::new(directory).join(&folder_name);
                fs::create_dir_all(&target_dir)?;
                let target_path = target_dir.join(path.file_name().unwrap());
                fs::rename(path, target_path)?;
            }
        }
    }
    Ok(())
}

pub fn delete_temp_files(directory: &str) -> std::io::Result<()> {
    for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "tmp" || extension == "temp" {
                    fs::remove_file(path)?;
                    println!("Удален временный файл: {:?}", path);
                }
            }
        }
    }
    Ok(())
}
