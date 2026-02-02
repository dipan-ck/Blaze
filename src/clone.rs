use reqwest::Client;

pub async fn clone(base_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let hash = reference_discovery(base_url).await.unwrap();
    println!("hash is: {hash}");

    let bytes = send_want(base_url, &hash).await.unwrap();

    Ok(())
}

async fn reference_discovery(base_url: &str) -> Result<String, reqwest::Error> {
    let url = format!("{}/info/refs?service=git-upload-pack", base_url);
    let client = Client::new();
    let resp = client
        .get(url)
        .header("Accept", "application/x-git-upload-pack-advertisement")
        .send()
        .await
        .unwrap();

    let bytes = resp.bytes().await.unwrap().to_vec();
    Ok(extract_hash(bytes))
}

/*

This is the Response Structure form GitHub:

001e# service=git-upload-pack\n

0000

0159e6811516be17b2ef9a12c2d89343d1b357fa994d HEAD\0multi_ack thin-pack side-band side-band-64k ofs-delta shallow deepen-since
deepen-not deepen-relative no-progress include-tag multi_ack_detailed allow-tip-sha1-in-want allow-reachable-sha1-in-want no-done
symref=HEAD:refs/heads/main filter object-format=sha1 agent=git/github-bdd2406f1b30-Linux\n003de6811516be17b2ef9a12c2d89343d1b357fa994d
refs/heads/main\n003e0646ecd760a21a4e8d055d55359f0f35b6ad15c6 refs/pull/1/head\n

0000

 */

fn extract_hash(response_bytes: Vec<u8>) -> String {
    let mut pos = 0;

    let mut skip_len =
        usize::from_str_radix(str::from_utf8(&response_bytes[pos..4]).unwrap(), 16).unwrap();
    //skipping the first line and the 0000 end
    pos += skip_len + 4;

    skip_len =
        usize::from_str_radix(str::from_utf8(&response_bytes[pos..pos + 4]).unwrap(), 16).unwrap();

    let payload = &response_bytes[pos + 4..pos + skip_len];

    let hash_end_index = payload.iter().position(|&b| b == b' ' || b == 0).unwrap();

    String::from_utf8(payload[..hash_end_index].to_vec()).unwrap()
}

async fn send_want(base_url: &str, hash: &str) -> Result<Vec<u8>, reqwest::Error> {
    let client = Client::new();
    let url = format!("{}/git-upload-pack", base_url);

    // Include capabilities in the first want line
    let want_line = format!(
        "want {} multi_ack_detailed side-band-64k thin-pack ofs-delta agent=rust-git/0.1\n",
        hash
    );
    let want_line_pkt = format!("{:04x}{}", want_line.len() + 4, want_line);

    let flush_pkt = "0000";

    let done_line = "done\n";
    let done_pkt = format!("{:04x}{}", done_line.len() + 4, done_line);

    // The body should be: want-pkt, flush, done-pkt
    let body = format!("{}{}{}", want_line_pkt, flush_pkt, done_pkt);

    let resp = client
        .post(&url)
        .header("Content-Type", "application/x-git-upload-pack-request")
        .header("Accept", "application/x-git-upload-pack-result")
        .body(body.into_bytes())
        .send()
        .await?;

    let bytes = resp.bytes().await?;
    Ok(bytes.to_vec())
}
