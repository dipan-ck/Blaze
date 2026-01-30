use std::fs;

pub fn init() {
    fs::create_dir(".blitz").unwrap();
    fs::create_dir(".blitz/objects").unwrap();
    fs::create_dir(".blitz/refs").unwrap();
    fs::write(".blitz/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}
