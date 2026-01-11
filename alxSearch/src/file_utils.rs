use std::fs;
use std::path::Path;

pub fn list_text_files(dir: &Path, max_size: u64) -> Vec<String> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                files.extend(list_text_files(&path, max_size));
            } else if path.is_file() {
                if let Ok(meta) = fs::metadata(&path) {
                    if meta.len() <= max_size && is_text_file(&path) {
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    files
}

fn is_text_file(path: &Path) -> bool {
    if let Ok(data) = fs::read(path) {
        !data.contains(&0)
    } else { false }
}
