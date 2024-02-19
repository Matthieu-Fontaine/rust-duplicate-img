# Rust Duplicate Image Finder

This is a Rust project that helps you find duplicated photos by hashing and comparing them.

## How it works

1. The program takes a directory as input and scans all the image files in that directory.
2. For each image file, it calculates a hash value using a sha256 algorithm.
3. The program then compares the hash vaues of all the image files to identify duplicates.
4. Duplicated photos are displayed as a list, making it easy to identify and manage them.

## Features

- Fast and efficient duplicate detection using hashing algorithms.
- Supports various image file formats, including JPEG, PNG, and GIF.
- Provides a user-friendly interface to view and manage duplicated photos.
- Customizable options for hash algorithm selection and comparison threshold.

## Getting Started

To get started with the Rust Duplicate Image Finder, follow these steps:

1. Clone the repository: `git clone https://github.com/your-username/rust-duplicate-img.git`
2. Install Rust and Cargo if you haven't already.
3. Navigate to the project directory: `cd rust-duplicate-img`
4. Build the project: `cargo build`
5. Run the program: `cargo run -- <directory-path>`

Replace `<directory-path>` with the path to the directory you want to scan for duplicated photos.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.
