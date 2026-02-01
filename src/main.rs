use std::env;

use blaze::run;

fn main() {
    let args = env::args().collect();
    run(args);
}
