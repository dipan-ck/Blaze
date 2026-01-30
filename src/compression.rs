use flate2::read::ZlibDecoder;
use flate2::{Compression, write::ZlibEncoder};
use std::io::{Read, Write};

/*
 This function takes the decompressed blob and persoms compression in it using the flate2 create
 which provides a zlib encoder the compressed blob is a vector of raw bytes
*/
pub fn compress_blob(blob: &Vec<u8>) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&blob).expect("zlib write failed");
    let compressed_blob = encoder.finish().expect("zlib finish failed");
    return compressed_blob;
}

/*
this used the zlib decompressor to decompress the raw bytes returned by the find_blob_by_hash function. It retuens a
decompress verdsion of the blob which is also a vector of bytes
*/
pub fn decompress_blob(compressed: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed = Vec::new();
    decoder
        .read_to_end(&mut decompressed)
        .expect("zlib decompress failed");
    return decompressed;
}
