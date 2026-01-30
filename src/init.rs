use std::fs;

/*
 Creates the .blitz folder and inside the .bliz it creates a objects folder which will
 store blobs, trees etc. and a refs folder and also creates a HEAD file if it doesnt exixt
and writes "ref: refs/heads/main\n" iin the file
*/

pub fn init() {
    fs::create_dir(".blitz").unwrap();
    fs::create_dir(".blitz/objects").unwrap();
    fs::create_dir(".blitz/refs").unwrap();
    fs::write(".blitz/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}
