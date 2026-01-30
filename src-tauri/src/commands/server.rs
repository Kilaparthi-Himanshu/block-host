#![allow(dead_code, unused, unused_imports)]

use reqwest::Client;
use serde::Deserialize;
use std::{fs, path::PathBuf};

use crate::utils::path::servers_dir;

#[derive(Deserialize, Debug)]
struct VersionDetails {
    downloads: Downloads,
}

#[derive(Deserialize, Debug)]
struct Downloads {
    server: DownloadInfo,
}

#[derive(Deserialize, Debug)]
struct DownloadInfo {
    url: String,
}

#[tauri::command]
pub async fn create_server(name: String, version: String) -> Result<String, String> {
    // Create server folder
    let mut server_path = servers_dir();
    server_path.push(&version);
    server_path.push(&name);

    println!("{:?}", server_path);

    if server_path.exists() {
        return Err("Server already exists".into());
    }

    fs::create_dir_all(&server_path).map_err(|e| e.to_string())?;

    // Fetch version manifest
    let client = Client::new();

    let manifest: serde_json::Value = client
        .get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let versions = manifest["versions"].as_array().ok_or("Invalid Manifest")?;

    let version_url = versions
        .iter()
        .find(|v| v["id"] == version)
        .and_then(|v| v["url"].as_str())
        .ok_or("Version not found")?;

    // Fetch version details
    let details: VersionDetails = client
        .get(version_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    println!("{:?}", details);

    let server_url = details.downloads.server.url;

    // Download server.jar
    let jar_path = server_path.join("server.jar");
    let bytes = client
        .get(server_url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    fs::write(&jar_path, bytes).map_err(|e| e.to_string())?;

    // Write eula.txt
    fs::write(server_path.join("eula.txt"), "eula=true\n").map_err(|e| e.to_string())?;

    Ok(format!(
        "Server {} ({}) created successfully",
        name, version
    ))
}
