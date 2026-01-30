use std::{
    fs,
    io::{self, Write},
};

use crate::compression::decompress;

pub fn ls_tree(hash: &str) {
    let dir = &hash[0..2];
    let file = &hash[2..];
    let object_path = format!(".blitz/objects/{dir}/{file}");

    let raw_bytes = fs::read(object_path).unwrap();
    let decompressed_bytes = decompress(&raw_bytes);
    let mut headerless_content = Vec::new();

    for (index, byte) in decompressed_bytes.iter().enumerate() {
        if *byte == 0 {
            headerless_content = decompressed_bytes[index + 1..].to_vec();
            break;
        }
    }

    render_tree(headerless_content);
}

fn render_tree(file_bytes: Vec<u8>) {
    let mut output: Vec<u8> = Vec::new();

    let mut pos: usize = 0;

    loop {
        if pos >= file_bytes.len() {
            break;
        }

        if file_bytes[pos] != b' ' {
            pos += 1;
        } else {
            pos += 1;
            while file_bytes[pos] != 0 {
                output.push(file_bytes[pos]);
                pos += 1;
            }
            output.push(b'\n');
            pos += 21;
        }
    }

    io::stdout().write(&output).unwrap();
}
