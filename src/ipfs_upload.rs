extern crate ipfs_api;

use ipfs_api::{IpfsClient, Error, IpfsApi};
use std::fs::File;
use std::path::Path;
use futures::TryStreamExt;
use sha2::{Digest, Sha256};
use std::io::Read;

// Function to calculate the SHA-256 hash of file contents
fn calculate_file_hash(file_path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);
    Ok(hex::encode(hasher.finalize()))
}

// Function to upload a file to IPFS and get the hash
pub async fn upload_to_ipfs(file_path: &Path) -> Result<String, Error> {
    // Create an IPFS client
    println!("Uploading to IPFS");
    let client = IpfsClient::default();

    // Calculate and print the hash of the file before uploading
    match calculate_file_hash(file_path) {
        Ok(hash) => println!("File hash before upload: {}", hash),
        Err(e) => eprintln!("Error calculating file hash: {}", e),
    }

    // Open the file
    let file = File::open(file_path).expect("could not read source file");

    // Upload the file to IPFS
    match client.add(file).await {
        Ok(res) => {
            println!("Uploaded file hash: {}", res.hash);
            Ok(res.hash)
        },
        Err(e) => Err(e),
    }
}

// Function to download a file from IPFS using its hash
pub async fn download_file_from_ipfs(hash: &str) -> Result<Vec<u8>, Error> {
    let client = IpfsClient::default();

    let result = client.cat(hash)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await?;

    // Calculate and print the hash of the downloaded data
    let mut hasher = Sha256::new();
    hasher.update(&result);
    let downloaded_hash = hex::encode(hasher.finalize());
    println!("Downloaded data hash: {}", downloaded_hash);

    Ok(result)
}
