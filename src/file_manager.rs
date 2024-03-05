#![warn(clippy::all, clippy::pedantic)]
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::file_manager::hash::sha256_comparison_file;

mod hash;

fn copy_file(src: &Path, dst: &Path) {
    fs::copy(src, dst).unwrap();
}

fn file_exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

fn dir_exists(path: &Path) {
    if fs::metadata(path).is_err() {
        fs::create_dir(path).expect("Failed to create directory");
    }
}

fn folder_checks(from_path: &Path, to_path: &Path) {
    for object in WalkDir::new(from_path)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        let object_path = object.into_path();
        let object_relative_path = object_path.strip_prefix(from_path).unwrap();
        if object_path.is_file() {
            let save_file_path = to_path.join(object_relative_path);
            if !file_exists(&save_file_path) {
                dir_exists(&save_file_path.with_file_name(""));
                copy_file(&object_path, &save_file_path);
            };
            if !sha256_comparison_file(&object_path, &save_file_path) {
                copy_file(&object_path, &save_file_path);
            }
        };
    }
}

pub fn start_copying(from_path: &Path, to_path: &Path) {
    if !from_path.is_dir() {
        panic!("The folder to copy from does not exist")
    };
    dir_exists(to_path);
    folder_checks(from_path, to_path);
}