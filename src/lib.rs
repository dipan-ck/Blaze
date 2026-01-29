use std::fs;

use sha1::{Digest, Sha1};

pub fn init() {
    fs::create_dir(".blitz").unwrap();
    fs::create_dir(".blitz/objects").unwrap();
    fs::create_dir(".blitz/refs").unwrap();
    fs::write(".blitz/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

pub fn create_blob_object(args: &Vec<String>) {
    if args.len() != 3 {
        panic!("file not provided");
    }

    let path = &args[2];
    let file_content = fs::read(path).expect("failed to read file");
    let mut blob = Vec::new();

    blob.extend_from_slice(b"blob ");
    blob.extend_from_slice(file_content.len().to_string().as_bytes());
    blob.push(0);
    blob.extend_from_slice(&file_content);

    let mut hasher = Sha1::new();
    hasher.update(&blob);
    let hash = hasher.finalize();

    let hex = format!("{:x}", hash);
    println!("hash is : {hex}",);
}
