#![warn(clippy::all, clippy::pedantic)]
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

fn sha256_digest(path: &Path) -> String {
    let input = File::open(path).unwrap();
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    format!("{digest:X}")
}

pub fn sha256_comparison_file(first_path: &Path, second_file: &Path) -> bool {
    sha256_digest(first_path) == sha256_digest(second_file)
}
