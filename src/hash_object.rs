use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{fs, io::Write};

/*
 The create_blob takes the file_content which is a array of raw bytes. Asd per git inner working it adds
 " blob <size>\0<content>" at the starting of the content and compressed it so that when git decompresses it later it knows what object it is and its length
 so, we created a blob vector added the "blob " as a bytes then added 0 which is a null byte and then we also push the file content to the bloblvector
 so, this function just creates the final blob before compression
*/
fn create_blob(file_content: Vec<u8>) -> Vec<u8> {
    let mut blob = Vec::new();

    blob.extend_from_slice(b"blob ");
    blob.extend_from_slice(file_content.len().to_string().as_bytes());
    blob.push(0);
    blob.extend_from_slice(&file_content);

    return blob;
}

/*
This function takes the decompressed blob and hashes it using the SHA1 algorithm now hasher.finalize return a
vector of raw bytes so we use the format macro to convert it to a hex and return the hex version of the hash
 */
fn hash_blob(blob: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(&blob);
    let hash = hasher.finalize();
    let hex = format!("{:x}", hash);
    return hex;
}

/*
 This function takes the decompressed blob and persoms compression in it using the flate2 create
 which provides a zlib encoder the compressed blob is a vector of raw bytes
*/
fn compress_blob(blob: &Vec<u8>) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob).expect("zlib write failed");
    let compressed_blob = encoder.finish().expect("zlib finish failed");
    return compressed_blob;
}

/*
 If a hash for a object is a7b9a1dcc3f3f148342270696dbbbea060b9f6b4  the path for that object is .blitz/objects/a7/b9a1dcc3f3f148342270696dbbbea060b9f6b4 so
 this function takes the hash and creates the write path and writes the blobl object.
*/
fn create_object_dir(hash: &String, compressed_blob: Vec<u8>) {
    let folder = &hash[0..2];
    let file = &hash[2..];

    let dir_path = format!(".blitz/objects/{folder}");
    let write_path = format!(".blitz/objects/{folder}/{file}");

    fs::create_dir_all(dir_path).expect("failed to create the blob folder");
    fs::write(write_path, compressed_blob).expect("something went wrong while creating the object");

    println!("hash is : {hash}");
}

/*
 cargo run -- hash-object   hello.txt in this command we run this function this function doesn't create a blobl object in the
 objects dir it only shows the user the hash generated from the contents of the file provided there is no compression done
*/
pub fn hash_object(path: &str) {
    let file_content = fs::read(path).expect("failed to read file");
    let blob = create_blob(file_content);
    let hash = hash_blob(&blob);
    println!("{hash}");
}

/*
 cargo run -- hash-object -w  hello.txt  in this command we run this function this creates the hash and also does the
 compression of the blob and sstoring the blobl Object
*/
pub fn create_blob_object(path: &str) {
    let file_content = fs::read(path).expect("failed to read file");
    let blob = create_blob(file_content);
    let hash = hash_blob(&blob);
    let compressed_blob = compress_blob(&blob);
    create_object_dir(&hash, compressed_blob);
}
