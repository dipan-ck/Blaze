use blaze::{create_blob_object, init};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "init" {
        init();
    } else if args[1] == "hash" {
        create_blob_object(&args);
    }
}
