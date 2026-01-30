use std::{fs, io::Write};

use crate::blob_object::create_blob_object;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

struct Entry {
    mode: String,
    name: String,
    hash: Vec<u8>,
}

impl Entry {
    fn new(path: &str, name: String, hash: Vec<u8>) -> Entry {
        Entry {
            mode: get_file_mode(path),
            name,
            hash,
        }
    }
}

fn get_file_mode(path: &str) -> String {
    let metadata = fs::metadata(path).unwrap();
    if metadata.is_dir() {
        "40000".to_string()
    } else {
        "100644".to_string()
    }
}

pub fn tree(dir_path: &str) {
    let (hex, _) = create_tree_object(dir_path);
    println!("{hex}");
}

pub fn create_tree_object(dir_path: &str) -> (String, Vec<u8>) {
    let reader = fs::read_dir(dir_path).unwrap();
    let mut tree_content: Vec<Entry> = Vec::new();

    for entry in reader {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        if !metadata.is_dir() {
            let path = entry.path();
            let file_name = entry.file_name();

            let (_, hash_byte) = create_blob_object(path.to_str().unwrap());
            tree_content.push(Entry::new(
                path.to_str().unwrap(),
                file_name.to_str().unwrap().to_string(),
                hash_byte,
            ));
        } else {
            let path = entry.path();
            let file_name = entry.file_name();
            let (_, hash_bytes) = create_tree_object(path.to_str().unwrap());
            tree_content.push(Entry::new(
                path.to_str().unwrap(),
                file_name.to_str().unwrap().to_string(),
                hash_bytes,
            ));
        }
    }

    tree_content.sort_by(|a, b| a.name.cmp(&b.name));

    let mut tree_object = Vec::new();

    let mut tree_object_content = Vec::new();

    for e in &tree_content {
        let mode = e.mode.to_string();
        tree_object_content.extend_from_slice(mode.as_bytes());
        tree_object_content.extend_from_slice(b" ");
        tree_object_content.extend_from_slice(e.name.as_bytes());
        tree_object_content.push(0);
        tree_object_content.extend_from_slice(&e.hash);
    }

    tree_object.extend_from_slice(b"tree ");
    tree_object.extend_from_slice(tree_object_content.len().to_string().as_bytes());
    tree_object.push(0);
    tree_object.extend_from_slice(&tree_object_content);

    let mut hasher = Sha1::new();
    hasher.update(&tree_object);
    let hash = hasher.finalize();
    let hex = format!("{:x}", hash);

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&tree_object).expect("zlib write failed");
    let compressed_blob = encoder.finish().expect("zlib finish failed");

    let folder = &hex[0..2];
    let file = &hex[2..];
    let dir_path = format!(".blitz/objects/{folder}");
    let write_path = format!(".blitz/objects/{folder}/{file}");

    fs::create_dir_all(dir_path).expect("failed to create the blob folder");
    fs::write(write_path, compressed_blob).expect("something went wrong while creating the object");

    (hex, hash.to_vec())
}
