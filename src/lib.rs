use crate::{
    blob_object::{create_blob_object, hash_object},
    cat_file::cat_file,
    commit_tree::{commit_tree, initial_commit_tree},
    init::init,
    ls_tree::ls_tree,
    restore::restore,
    tree_object::write_tree,
};

pub mod blob_object;
pub mod cat_file;
pub mod commit_tree;
pub mod compression;
pub mod init;
pub mod ls_tree;
pub mod restore;
pub mod tree_object;

/*

SUPPORTED COMMANDS:

 cargo run -- init  -> Created the .blitz folder

 cargo run -- hash-object   test_repo/file.txt  -> Takes the file path and creates a SHA1 Hash from the content of the file

 cargo run -- hash-object -w test_repo/file.txt -> Creates the Hash compressed the content and creates the blob Object

 cargo run -- cat-file -p a7b9a1dcc3f3f148342270696dbbbea060b9f6b4   -> Gets the blobl Object from hash decompressed it and stdout's the output

  cargo run -- write-tree test_repo -> Takes the current snapshor of the directory and creates the Tree Object

  cargo run ls-tree --name-only a798152e791914cc4ecf875bc2a3fa97be374717  -> Takes a Tree Object Hash and prints all the contnts of that tree object

 cargo run -- commit-tree a798152e791914cc4ecf875bc2a3fa97be374717 -m "first commit"

 cargo run -- restore test_repo

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
        [
            _,
            "commit-tree",
            tree_hash,
            "-p",
            parent_commit_hash,
            "-m",
            commit_message,
        ] => commit_tree(tree_hash, parent_commit_hash, commit_message),
        [_, "restore", root_path] => restore(root_path),

        _ => eprintln!("unknown command"),
    }
}
