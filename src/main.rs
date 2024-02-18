use std::env;
use std::fs;
use std::io;
use walkdir::WalkDir;

mod custom_file;

fn main() {
    let current_dir = env::current_dir();

    let current_path = match current_dir {
        Ok(dir) => {
            // Successfully obtained the current directory
            println!("Current directory: {:?}", dir);
            dir
        }
        Err(err) => {
            // Handle the error case
            eprintln!("Error getting current directory: {}", err);
            // You can choose to return or panic, or take other actions based on the error
            // For now, let's use the current directory as a default value
            std::env::current_dir().expect("Failed to get current directory")
        }
    };

    println!("Enter the path to the directory you want to scan: ");

    let mut input_path = String::new();

    io::stdin()
        .read_line(&mut input_path)
        .expect("Failed to read line");

    let input_path = input_path.trim();

    // Concatenate the current directory with the user-input path
    let full_path = current_path.join(input_path);

    println!("Full path to scan: {:?}", full_path);

    let mut custom_files = list_files_recursively_v2(&full_path);

    // for file in custom_files {
    //     println!("{:?}", file.path);
    //     println!("{:?}", file.filename);
    //     println!("{:?}", file.extension);
    //     println!("{:?}", file.size);
    //     println!("{:?}", file.hash);
    //     println!("{:?}", file.duplicate);
    // }

    println!("Total files: {}", custom_files.len());

    // Compare all hashes to find duplicates
    for i in 0..custom_files.len() {
        for j in 0..custom_files.len() {
            if i != j && custom_files[i].hash == custom_files[j].hash {
                custom_files[i].duplicate = true;
            }
        }
    }

    // Print duplicates
    for file in &custom_files {
        if file.duplicate {
            println!("{:?}", file.path);
        }
    }



}

fn list_files_recursively_v2(dir: &std::path::Path) -> Vec<custom_file::customFile> {
    let mut custom_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        println!("{:?}", path);

        if path.is_file() {
            let custom_file = custom_file::customFile::new(path.to_str().unwrap());
            custom_files.push(custom_file);
        }
    }

    custom_files
}
