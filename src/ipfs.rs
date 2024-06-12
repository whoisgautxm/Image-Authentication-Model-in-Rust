use reqwest::multipart;
use std::path::Path;
use tokio;

pub async fn upload_to_ipfs(file_path: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://localhost:5001/api/v0/add"; // Change to your IPFS node address

    let form = multipart::Form::new()
        .file("file", file_path).expect("Failed to create multipart form");

    let response = client.post(url)
        .multipart(form)
        .send()
        .await?;

    let json: serde_json::Value = response.json().await?;
    Ok(json["Hash"].as_str().unwrap().to_string())
}
