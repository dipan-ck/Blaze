use std::{fs, io::Write};

use flate2::{Compression, write::ZlibEncoder};
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

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob).expect("zlib write failed");
    let compressed_blob = encoder.finish().expect("zlib finish failed");

    let folder = &hex[0..2];
    let file = &hex[2..];

    let dir_path = format!(".blitz/objects/{folder}");
    let write_path = format!(".blitz/objects/{folder}/{file}");

    fs::create_dir_all(dir_path).expect("failed to create the blob folder");
    fs::write(write_path, compressed_blob).expect("something went wrong while creating the object");

    println!("hash is : {hex}");
}
