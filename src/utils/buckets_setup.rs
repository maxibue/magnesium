use std::fs;
use std::path::Path;

pub fn buckets_setup(parent_directory: String, buckets: Vec<String>) {
    println!("Starting to setup buckets:\n");

    let mut count: i128 = 0;
    let parent_path = Path::new(&parent_directory);
    if parent_path.exists() && parent_path.is_dir() {
        println!("[/] Parent directory '{}' not created because it already existed.", parent_directory);
    } else {
        count += 1;
        match fs::create_dir(&parent_path) {
            Ok(_) => println!("[+] Parent directory '{}' created.", parent_directory),
            Err(e) => println!("[!] An error occurred while creating the parent directory '{}': {}", parent_directory, e),
        }
    }
    for bucket in buckets {
        let path = Path::new(&parent_directory).join(&bucket);
        if path.exists() && path.is_dir() {
            println!("[/] Directory '{}' not created because it already existed.", bucket);
        } else {
            count += 1;
            match fs::create_dir(&path) {
                Ok(_) => println!("[+] Directory '{}' was created.", bucket),
                Err(e) => println!("[!] An error occurred while creating folder '{}': {}", bucket, e),
            }
        }
    }

    println!("\nNewly created directories: {}", count);
}