use std::fs;

use crate::{blob_object::hash, compression::compress};

//for the first commit we use this function where we dont add a parent hash in the body  of the commit Object
pub fn initial_commit_tree(tree_hash: &str, commit_message: &str) {
    let body = format!(
        "tree {}\n\
author John Doe <john@example.com> 1234567890 +0000\n\
committer John Doe <john@example.com> 1234567890 +0000\n\
\n\
{}\n",
        tree_hash, commit_message
    );

    let header = format!("commit {}\0", body.len());

    let mut object = Vec::new();
    object.extend_from_slice(header.as_bytes());
    object.extend_from_slice(body.as_bytes());

    let (hex, _) = hash(&object);
    let compressed = compress(&object);

    let folder = &hex[0..2];
    let file = &hex[2..];
    let dir_path = format!(".blitz/objects/{folder}");
    let write_path = format!(".blitz/objects/{folder}/{file}");

    fs::create_dir_all(dir_path).unwrap();
    fs::write(write_path, compressed).unwrap();

    make_commit_head(&hex);

    println!("{hex}");
}

//This is used after the initial commit as per the current implementation user needs to pass the previous commit hash along with the tree hash
pub fn commit_tree(tree_sha: &str, parent_commit_sha: &str, commit_message: &str) {
    let body = format!(
        "tree {}\n\
parent {}\n\
author John Doe <john@example.com> 1234567890 +0000\n\
committer John Doe <john@example.com> 1234567890 +0000\n\
\n\
{}\n",
        tree_sha, parent_commit_sha, commit_message
    );

    let header = format!("commit {}\0", body.len());

    let mut object = Vec::new();
    object.extend_from_slice(header.as_bytes());
    object.extend_from_slice(body.as_bytes());

    let (hex, _) = hash(&object);
    let compressed = compress(&object);

    let folder = &hex[0..2];
    let file = &hex[2..];
    let dir_path = format!(".blitz/objects/{folder}");
    let write_path = format!(".blitz/objects/{folder}/{file}");

    fs::create_dir_all(dir_path).unwrap();
    fs::write(write_path, compressed).unwrap();

    make_commit_head(&hex);

    println!("{hex}");
}

fn make_commit_head(hex: &str) {
    fs::write(".blitz/HEAD", hex).unwrap();
}
