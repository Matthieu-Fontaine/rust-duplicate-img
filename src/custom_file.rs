use std::path::Path;
use std::fs;
use std::fs::File;
use sha2::{Sha256, Digest};
use std::io::Read;
use rfd::FileDialog;

pub struct CustomFile {
    pub path: String,
    pub filename: String,
    pub extension: String,
    pub size: u64,
    pub hash: String,
    pub duplicate: bool,
}

impl CustomFile {
    pub fn new(path: &str) -> CustomFile {
        CustomFile {
            path: path.to_string(),
            filename: extract_name(path),
            extension: extract_extension(path),
            size: extract_size(path),
            hash: create_hash(path),
            duplicate: false,
        }
    }
}

fn extract_name(path: &str) -> String {
    let path = Path::new(path);
    match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => String::from(""),
    }
}

fn extract_extension(path: &str) -> String {
    let path = Path::new(path);
    match path.extension() {
        Some(ext) => ext.to_string_lossy().to_string(),
        None => String::from(""),
    }
}

fn extract_size(path: &str) -> u64 {
    let metadata = fs::metadata(path).unwrap();
    metadata.len()
}

fn create_hash(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect()
}