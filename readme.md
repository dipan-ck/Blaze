# A Toy Implementation of Git in Rust

I have this thing where, until I understand how something really works internally, I never feel like I truly understand the technology—and this project is no exception.

I’ve been using Git since I started coding, and ever since then I’ve had a strong desire to understand how it actually works under the hood and eventually build my own version of it.

I also decided that if I were going to attempt building Git, I should do it using a systems-level programming language. Most of my experience has been with TypeScript, but after learning Rust, I fell in love with systems programming, and that felt like the right moment to finally take this on.

So here it is—a toy implementation of Git written in Rust, built for learning and curiosity rather than practicality.


## Commands Blitz Supports

### init
Initialize a new .blitz repository in the current directory.
```bash
cargo run -- init
```

### hash-object
Compute the hash of a file without writing it to the .blitz/objects directory
```bash
cargo run -- hash-object <file_path> 
```

Write a file to the .blitz/objects directory and return its hash:
```bash
cargo run -- hash-object -w <file_path> 
```

### cat-file
Display the contents of a Git object.
```bash
blitz cat-file -p <object_hash>
```

### write-tree
Creates a tree object from the current directory structure.
```bash
blitz write-tree <root_path>
```

### ls-tree
List the contents of a tree object.
```bash
blitz ls-tree --name-only <tree_hash>
```

## commit-tree
Create a new commit object.

Initial commit (no parent):
```bash
blitz commit-tree <tree_hash>  -m "first commit"
```

Commit with parent:
```bash
blitz commit-tree <tree_hash> -p <parent_commit_hash>  -m "second commit"
```

### `restore`
Restore working directory files from the latest commit tree.
```bash
blitz restore <root_path>
```
