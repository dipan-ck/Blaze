use crate::{
    blob_object::{create_blob_object, hash_object},
    cat_file::cat_file,
    commit_tree::initial_commit_tree,
    init::init,
    ls_tree::ls_tree,
    tree_object::write_tree,
};

pub mod blob_object;
pub mod cat_file;
pub mod commit_tree;
pub mod compression;
pub mod init;
pub mod ls_tree;
pub mod tree_object;

/*

SUPPORTED COMMANDS:

 cargo run -- init  -> Created the .blitz folder

 cargo run -- hash-object   hello.txt   -> Takes the file path and creates a SHA1 Hash from the content of the file

 cargo run -- hash-object -w  hello.txt -> Creates the Hash compressed the content and creates the blob Object

 cargo run -- cat-file -p a7b9a1dcc3f3f148342270696dbbbea060b9f6b4   -> Gets the blobl Object from hash decompressed it and stdout's the output


 cargo run -- commit-tree 31020dd853e5a1dbc1d5cec863744a2f0660e852 -m "first commit"

 */

pub fn run(args: Vec<String>) {
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match args.as_slice() {
        [_, "init"] => init(),
        [_, "hash-object", path] => hash_object(path),
        [_, "hash-object", "-w", path] => {
            let (hash, _) = create_blob_object(path);
            println!("{hash}");
        }
        [_, "cat-file", "-p", hash] => cat_file(hash),
        [_, "write-tree", path] => write_tree(path),
        [_, "ls-tree", "--name-only", hash] => ls_tree(hash),
        [_, "commit-tree", tree_hash, "-m", commit_message] => {
            initial_commit_tree(tree_hash, commit_message)
        }
        _ => eprintln!("unknown command"),
    }
}
