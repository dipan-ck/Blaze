use crate::{
    cat_file::cat_file,
    hash_object::{create_blob_object, hash_object},
    init::init,
};

pub mod cat_file;
pub mod hash_object;
pub mod init;

pub fn run(args: Vec<String>) {
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match args.as_slice() {
        [_, "init"] => init(),
        [_, "hash-object", path] => hash_object(path),
        [_, "hash-object", "-w", path] => create_blob_object(path),
        [_, "cat-file", "-p", hash] => cat_file(hash),
        _ => eprintln!("unknown command"),
    }
}
