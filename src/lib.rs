use crate::{
    cat_file::cat_file,
    hash_object::{create_blob_object, hash_object},
    init::init,
};

pub mod cat_file;
pub mod hash_object;
pub mod init;

/*

SUPPORTED COMMANDS:

 cargo run -- init  -> Created the .blitz folder

 cargo run -- hash-object   hello.txt   -> Takes the file path and creates a SHA1 Hash from the content of the file

 cargo run -- hash-object -w  hello.txt -> Creates the Hash compressed the content and creates the blob Object

 cargo run -- cat-file -p a7b9a1dcc3f3f148342270696dbbbea060b9f6b4   -> Gets the blobl Object from hash decompressed it and stdout's the output


 */

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
