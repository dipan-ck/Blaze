use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{fs, io::Write};

fn create_blob(file_content: Vec<u8>) -> Vec<u8> {
    let mut blob = Vec::new();

    blob.extend_from_slice(b"blob ");
    blob.extend_from_slice(file_content.len().to_string().as_bytes());
    blob.push(0);
    blob.extend_from_slice(&file_content);

    return blob;
}

fn hash_blob(blob: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&blob);
    let hash = hasher.finalize();
    let hex = format!("{:x}", hash);
    return hex;
}

fn compress_blob(blob: &Vec<u8>) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob).expect("zlib write failed");
    let compressed_blob = encoder.finish().expect("zlib finish failed");
    return compressed_blob;
}

fn create_object_dir(hash: &String, compressed_blob: Vec<u8>) {
    let folder = &hash[0..2];
    let file = &hash[2..];

    let dir_path = format!(".blitz/objects/{folder}");
    let write_path = format!(".blitz/objects/{folder}/{file}");

    fs::create_dir_all(dir_path).expect("failed to create the blob folder");
    fs::write(write_path, compressed_blob).expect("something went wrong while creating the object");

    println!("hash is : {hash}");
}

pub fn hash_object(path: &str) {
    let file_content = fs::read(path).expect("failed to read file");
    let blob = create_blob(file_content);
    let hash = hash_blob(&blob);
    println!("{hash}");
}

pub fn create_blob_object(path: &str) {
    let file_content = fs::read(path).expect("failed to read file");
    let blob = create_blob(file_content);
    let hash = hash_blob(&blob);
    let compressed_blob = compress_blob(&blob);
    create_object_dir(&hash, compressed_blob);
}
