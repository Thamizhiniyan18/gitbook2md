use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub struct FileDetail {
    pub file_path: PathBuf,
    pub file_dir: PathBuf,
    pub output_assets_dir: PathBuf,
    pub output_file_path: PathBuf,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

pub fn find_md_files(source_dir: &PathBuf, output_dir: &PathBuf) -> Vec<FileDetail> {
    let walker = WalkDir::new(source_dir).into_iter();

    let mut md_files: Vec<FileDetail> = Vec::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();

        if entry.path().is_file()
            && entry.path().extension().and_then(|ext| ext.to_str()) == Some("md")
        {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path: PathBuf = entry.path().to_path_buf();
            let output_file_dir: PathBuf = output_dir.join(
                &file_path
                    .strip_prefix(source_dir)
                    .ok()
                    .and_then(|path| Some(path.with_extension("")))
                    .unwrap_or_else(|| {
                        eprintln!("Warning: Failed to process path {:?}", file_path);
                        PathBuf::new() // return an empty PathBuf if something goes wrong
                    }),
            );

            md_files.push(FileDetail {
                file_path,
                file_dir: entry.path().parent().unwrap().to_path_buf(),
                output_assets_dir: output_file_dir.join(Path::new("assets")),
                output_file_path: output_file_dir.join(Path::new(&file_name)),
            });
        }
    }

    return md_files;
}

pub fn create_output_directories(md_files: &Vec<FileDetail>) {

    for each in md_files {
        match create_dir_all(&each.output_assets_dir) {
            Ok(_) => {}
            Err(error) => panic!("Failed to create the directory: {}", error),
        }
    }
}
