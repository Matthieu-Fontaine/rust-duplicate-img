use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

extern crate gtk;

use gtk::prelude::*;
use gtk::{Builder, Button, FileChooserButton, Inhibit, Window};

mod custom_file;

fn main() {
    let builder = lauch_main_window();

    // Récupère le FileChooserButton à partir du fichier Glade
    let file_chooser_button: FileChooserButton = builder
        .get_object("folder_chooser_button")
        .expect("Couldn't get file_chooser_button");

    // Récupère le bouton "Lauch"
    let lauch_button: Button = builder
        .get_object("lauch_button")
        .expect("Couldn't get lauch_button");

    //lauch_button.set_sensitive(false);

    file_chooser_button.connect_file_set(move |file_chooser_button| {
        if let Some(_filename) = file_chooser_button.get_filename() {
            //lauch_button.set_sensitive(true);
        } else {
            //lauch_button.set_sensitive(false);
        }
    });

    lauch_button.connect_clicked(move |_| {
        if let Some(folder_path) = file_chooser_button.get_filename() {
            lauch(folder_path);
        }
    });

    gtk::main();
}

fn lauch(folder_path: PathBuf) {
    println!("Selected file: {:?}", folder_path);

    let dir = Path::new(&folder_path);

    println!("dir: {:?}", dir);

    let custom_files = list_files_recursively_v2(dir);

    let image_files = image_file_filter(custom_files);

    let mut image_files = image_files;

    find_duplicates(&mut image_files);

    for image_file in image_files {
        if image_file.duplicate {
            println!("Duplicate: {:?}", image_file.filename);
        }
    }

    gtk::main_quit();
}

fn lauch_main_window() -> Builder {
    gtk::init().expect("Failed to initialize GTK.");

    let glade_src = include_str!("ui/main_ui.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder
        .get_object("main_window")
        .expect("Couldn't get window");

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    builder
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

fn image_file_extension(file_extension: &str) -> bool {
    match file_extension {
        "jpg" => true,
        "jpeg" => true,
        "png" => true,
        "gif" => true,
        "bmp" => true,
        "tiff" => true,
        "webp" => true,
        "heif" => true,
        "heic" => true,
        "avif" => true,
        _ => false,
    }
}

fn image_file_filter(custom_files: Vec<custom_file::CustomFile>) -> Vec<custom_file::CustomFile> {
    let mut image_files = Vec::new();

    for custom_file in custom_files {
        if image_file_extension(&custom_file.extension) {
            image_files.push(custom_file);
        }
    }

    image_files
}

fn find_duplicates(image_files: &mut Vec<custom_file::CustomFile>) {
    let mut seen_hashes = HashSet::new();

    for file in image_files.iter_mut() {
        if seen_hashes.contains(&file.hash) {
            file.duplicate = true;
            // if let Some(original_file) = files.iter_mut().find(|&f| f.hash == file.hash) {
            //     original_file.duplicate = true;
            // }
        } else {
            seen_hashes.insert(file.hash.clone());
        }
    }
}
