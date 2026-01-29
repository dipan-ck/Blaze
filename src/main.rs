use blaze::init;
use std::env;

fn main() {
    let cmd: Vec<String> = env::args().collect();

    if cmd[1] == "init" {
        init();
    }
}
