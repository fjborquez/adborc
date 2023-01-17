use std::path::PathBuf;
use std::{env, io};
use std::fs::File;

pub fn create_path(path: Option<String>) -> PathBuf {
    if path.is_none() {
        return env::current_exe().unwrap().parent().unwrap().to_path_buf();
    }

    let temp_path = PathBuf::from(path.as_ref().unwrap());

    if !is_valid_path(&temp_path) {
        panic!("Invalid path specified");
    }

    return temp_path;
}

pub fn is_valid_path(path: &PathBuf) -> bool {
    path.exists() && path.is_dir()
}

pub fn create_file(path: PathBuf) -> File {
    let out_file = File::create(path.clone());
    check_file_error(&out_file, &path);

    return out_file.unwrap();
}

pub fn check_file_error(file: &io::Result<File>, path: &PathBuf) {
    if file.is_err() {
        panic!(
            "Error creating file: {}\n{}",
            path.display(),
            file.as_ref().unwrap_err()
        );
    }
}
