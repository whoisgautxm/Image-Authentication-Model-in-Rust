extern crate ipfs_api;

use ipfs_api::IpfsClient;
use std::fs::File;
use std::path::Path;
use std::error::Error;

// Function to upload a file to IPFS and get the hash
pub async fn upload_to_ipfs(file_path: &Path) -> Result<String, ipfs_api::request::Error> {
    // Create an IPFS client
    let client = IpfsClient::default();

    // Open the file
    let file = File::open(file_path).expect("could not read source file");

    // Upload the file to IPFS
    let res = client.add(file).await?;

    // Return the hash of the uploaded file
    Ok(res.hash)
}
