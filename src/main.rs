use rfd::FileDialog;
use walkdir::WalkDir;

mod custom_file;

fn main() {
    let folder = FileDialog::new().pick_folder();

    let folder_path = match folder {
        Some(path) => path,
        None => {
            println!("No folder selected");
            return;
        }
    };

    println!("{:?}", folder_path);

    let mut custom_files = list_files_recursively_v2(&folder_path);

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

fn list_files_recursively_v2(dir: &std::path::Path) -> Vec<custom_file::CustomFile> {
    let mut custom_files = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        // println!("{:?}", path);

        if path.is_file() {
            let custom_file = custom_file::CustomFile::new(path.to_str().unwrap());
            custom_files.push(custom_file);
        }
    }

    custom_files
}
