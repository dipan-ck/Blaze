use std::fs;

use crate::{cat_file::find_blob_by_hash, compression::decompress};

#[allow(unused)]
#[derive(Debug)]
struct Entry {
    mode: String,
    name: String,
    hash: Vec<u8>,
    hash_hex: String,
}

impl Entry {
    fn new(mode: String, name: String, hash: Vec<u8>, hash_hex: String) -> Entry {
        Entry {
            mode,
            name,
            hash,
            hash_hex,
        }
    }
}

#[allow(unused)]
pub fn restore(root_path: &str) {
    let latest_commit_hex_sha = get_latest_commit_hex_sha();
    let (root_tree_hex, root_tree_hash) = get_root_tree_hash_from_commit(&latest_commit_hex_sha);
    let root_tree_object = get_tree_object(&root_tree_hex);
    let decompressed_tree_object = decompress(&root_tree_object);
    let root_tree_entries = get_tree_entries(&decompressed_tree_object);

    fs::create_dir(root_path).unwrap();

    for entry in root_tree_entries {
        if entry.mode == "40000" {
            let path = format!("{root_path}/{}", &entry.name);
            recursive_builder(&path, &entry.hash_hex);
        } else {
            let file_path = format!("{root_path}/{}", entry.name);
            let file_content = file_content(&entry.hash_hex);
            fs::write(file_path, file_content).expect("error while writing content to file");
        }
    }

    println!("Restored successfully from the latest Commit");
}

fn get_latest_commit_hex_sha() -> String {
    let hex = fs::read_to_string(".blitz/HEAD").unwrap();
    hex
}

fn get_root_tree_hash_from_commit(commit_hex_sha: &str) -> (String, Vec<u8>) {
    let folder = &commit_hex_sha[0..2];
    let file = &commit_hex_sha[2..];
    let commit_object_path = format!(".blitz/objects/{folder}/{file}");

    let compressed_commit_bytes = fs::read(commit_object_path).unwrap();
    let decompressed_commit_bytes = decompress(&compressed_commit_bytes);

    let mut hash = Vec::new();

    let mut pos = 0;

    while decompressed_commit_bytes[pos] != 0 {
        pos += 1;
    }
    while decompressed_commit_bytes[pos] != b' ' {
        pos += 1;
    }

    pos += 1;

    while decompressed_commit_bytes[pos] != b'\n' {
        hash.push(decompressed_commit_bytes[pos]);
        pos += 1
    }
    let hex = String::from_utf8(hash.clone()).unwrap();
    (hex, hash)
}

fn get_tree_object(hash: &str) -> Vec<u8> {
    let folder = &hash[0..2];
    let file = &hash[2..];
    let tree_path = format!(".blitz/objects/{folder}/{file}");
    fs::read(tree_path).expect("Tree object doesn't exist")
}

fn recursive_builder(path: &str, hash: &str) {
    let tree_object = get_tree_object(hash);
    let decompressed_tree_object = decompress(&tree_object);
    let root_tree_entries = get_tree_entries(&decompressed_tree_object);

    fs::create_dir(path).unwrap();
    println!("recursive called");

    for entry in root_tree_entries {
        if entry.mode == "40000" {
            let path = format!("{path}/{}", &entry.name);
            recursive_builder(&path, hash);
        } else {
            let file_path = format!("{path}/{}", &entry.name);
            let file_content = file_content(&entry.hash_hex);
            fs::write(file_path, file_content).expect("error while writing content to file");
        }
    }
}

fn get_tree_entries(bytes: &Vec<u8>) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();

    let mut pos = 0;

    while bytes[pos] != 0 {
        pos += 1;
    }
    let headless_content = &bytes[pos + 1..];

    pos = 0;

    while pos < headless_content.len() {
        let mut mode = Vec::new();
        let mut name = Vec::new();
        let mut hash = Vec::new();

        while headless_content[pos] != b' ' {
            mode.push(headless_content[pos]);
            pos += 1;
        }

        pos += 1;

        while headless_content[pos] != 0 {
            name.push(headless_content[pos]);
            pos += 1;
        }

        pos += 1;
        hash.extend_from_slice(&headless_content[pos..pos + 20]);
        pos += 20;

        let hash_hex = hash
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();

        entries.push(Entry::new(
            String::from_utf8(mode).unwrap(),
            String::from_utf8(name).unwrap(),
            hash,
            hash_hex,
        ));
    }

    entries
}

pub fn file_content(hash: &str) -> Vec<u8> {
    let compressed_blob = find_blob_by_hash(hash);
    let decompressed_blob = decompress(&compressed_blob);

    let mut null_position: usize = 0;

    for (index, char) in decompressed_blob.iter().enumerate() {
        if *char == 0 {
            null_position = index;
            break;
        }
    }

    let content = &decompressed_blob[null_position + 1..];
    content.to_vec()
}
