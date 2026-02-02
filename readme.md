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
cargo run -- cat-file -p <object_hash>
```

### write-tree
Creates a tree object from the current directory structure.
```bash
cargo run -- write-tree <root_path>
```

### ls-tree
List the contents of a tree object.
```bash
cargo run -- ls-tree --name-only <tree_hash>
```

## commit-tree
Create a new commit object.

Initial commit (no parent):
```bash
cargo run -- commit-tree <tree_hash>  -m "first commit"
```

Commit with parent:
```bash
cargo run -- commit-tree <tree_hash> -p <parent_commit_hash>  -m "second commit"
```

### restore
Restore working directory files from the latest commit tree.
```bash
cargo run -- restore <root_path>
```

<br><br>

## How Git (and Blitz) Works
Git stores everything as objects in the `.git/objects` directory (`.blitz/objects` in our case). There are four types of objects:

- **Blob**: Stores file contents
- **Tree**: Represents a directory, containing references to blobs and other trees
- **Commit**: Points to a tree and contains metadata (author, message, parent commits)
- **Tag**: Points to a commit (not yet implemented in Blitz)

**In Blitz, all object types are implemented except for the Tag object.**

<br>

### Data Structures Used
- **Blob objects**: Raw file content compressed with zlib
- **Tree objects**: List of entries (mode, type, hash, name)
- **Commit objects**: Tree hash + parent hash + author + message + timestamp. In this Project we have hardcoded a dummy user data for commits.

<br>

### Content-Addressable Storage
Instead of storing files by name, Blitz (like Git) stores everything by the SHA-1 hash of its contents. The Hash is computed from the object's contents.

- **Deduplication**: Identical content always produces the same hash, so Git only stores it once, even if it appears in multiple files or commits
- **Integrity verification**: You can verify data hasn't been corrupted by recomputing the hash and comparing it to the stored one
- **Immutability**: Since the hash is derived from the content, you can't change an object without changing its hash—making the entire history tamper-evident

<br>

### How a Commit Works
1. Files are hashed and stored as blob objects
2. Directory structure is captured in tree objects
3. A commit object points to the root tree and contains metadata
4. .blitz/HEAD contains the hash of the latest Commit (but in case git it's a bit different)

<br>

### Directory Structure
```
.blitz/
├── objects/          # Object database (content-addressable)
│   ├── ab/
│   │   └── cdef123...  # Objects stored by hash prefix
├── refs/
│   └── heads/        # Branch references
└── HEAD              # Pointing to Latest Commit Hash
```
