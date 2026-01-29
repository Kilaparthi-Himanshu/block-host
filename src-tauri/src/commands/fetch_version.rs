use serde::{Deserialize, Serialize};
use serde_json::Value;

// #[derive(Deserialize, Serialize)]
// pub struct McVersion {
//     pub id: String,
//     pub r#type: String, // type is a reserved keyword in Rust, thats why r#
// }

#[tauri::command]
pub async fn get_mc_versions() -> Result<Vec<String>, String> {
    let resp: Value = reqwest::get(
        "https://launchermeta.mojang.com/mc/game/version_manifest.json"
    )
        .await.map_err(|e| e.to_string())?
        .json().await.map_err(|e| e.to_string())?;

    let versions = resp["versions"]
        .as_array()
        .ok_or("Invalid manifest")?;

    let releases: Vec<String> = versions
        .iter()
        .filter(|v| v["type"] == "release")
        .filter_map(|v| v["id"].as_str().map(String::from))
        .collect();

    Ok(releases)
}
