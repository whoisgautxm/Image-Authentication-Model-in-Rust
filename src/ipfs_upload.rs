extern crate ipfs_api;

use ipfs_api::IpfsClient;
use std::fs::File;
use std::path::Path;
use ipfs_api::Error;
use ipfs_api::IpfsApi;


// Function to upload a file to IPFS and get the hash
pub async fn upload_to_ipfs(file_path: &Path) -> Result<String, Error> {
    // Create an IPFS client
    println!("Uploading to IPFS");
    let client = IpfsClient::default();

    // Open the file
    let file = File::open(file_path).expect("could not read source file");
    
    
    // Upload the file to IPFS
 
    match client.add(file).await {
        Ok(res) => Ok(res.hash),
        Err(e) => Err(e),
    }
    
}