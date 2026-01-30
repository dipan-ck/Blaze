use flate2::read::ZlibDecoder;
use std::{
    fs,
    io::{self, Read, Write},
};

fn find_blob_by_hash(hash: &str) -> Vec<u8> {
    let folder = &hash[0..2];
    let file = &hash[2..];
    let path = format!(".blitz/objects/{folder}/{file}");
    let file = fs::read(path).expect("Unable to read Object from Hash");
    return file;
}

fn decompress_blob(compressed: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .expect("zlib decompress failed");
    return decompressed;
}

pub fn cat_file(hash: &str) {
    let compressed_blob = find_blob_by_hash(hash);
    let decompressed_blob = decompress_blob(&compressed_blob);

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
