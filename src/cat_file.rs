use std::{
    fs,
    io::{self, Write},
};

use crate::compression::decompress;

/*
 This function is responsible for finding a Blog Object file by the hash provided to it as i said earlier if a hash
 for a object is a7b9a1dcc3f3f148342270696dbbbea060b9f6b4  the path for that object is .blitz/objects/a7/b9a1dcc3f3f148342270696dbbbea060b9f6b4 so
 this function's logic is to get the file read the raw bytes and retuen the raw bytes vector
*/
fn find_blob_by_hash(hash: &str) -> Vec<u8> {
    let folder = &hash[0..2];
    let file = &hash[2..];
    let path = format!(".blitz/objects/{folder}/{file}");
    let file = fs::read(path).expect("Unable to read Object from Hash");
    return file;
}

/*
cargo run -- cat-file -p a7b9a1dcc3f3f148342270696dbbbea060b9f6b4  this function is the entry point for this command it takes the
decompressed blob and finds the null byte because while compression we added this at the start of the file then compressed it
blob <size>\0<content> but user should not see it so we fin the null byte and seperate wht's before that to get the content and then stdout the raw bytes
*/
pub fn cat_file(hash: &str) {
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
    io::stdout().write_all(content).unwrap();
}
