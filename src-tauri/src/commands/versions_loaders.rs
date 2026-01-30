use serde::{Deserialize, Serialize};
use serde_json::Value;

// #[derive(Deserialize, Serialize)]
// pub struct McVersion {
//     pub id: String,
//     pub r#type: String, // type is a reserved keyword in Rust, thats why r#
// }

#[tauri::command]
pub async fn get_mc_versions() -> Result<Vec<String>, String> {
    let resp: Value = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let versions = resp["versions"].as_array().ok_or("Invalid manifest")?;

    let releases: Vec<String> = versions
        .iter()
        .filter(|v| v["type"] == "release")
        .filter_map(|v| v["id"].as_str().map(String::from))
        .collect();

    Ok(releases)
}

use std::collections::HashSet;
use std::sync::Mutex;
use tokio::time::{sleep, Duration};

use crate::state::app_state::AppState;

pub struct LoaderSupportCache {
    pub fabric_versions: HashSet<String>,
    pub forge_versions: HashSet<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SupportLoaders {
    vanilla: bool,
    fabric: bool,
    forge: bool,
}

#[tauri::command]
pub async fn get_supported_loaders<'a>(
    version: String,
    state: tauri::State<'a, AppState>,
) -> Result<SupportLoaders, String> {
    loop {
        if let Some(cache) = state.loader_cache.lock().unwrap().as_ref() {
            return Ok(SupportLoaders {
                vanilla: true,
                fabric: cache.fabric_versions.contains(&version),
                forge: cache.forge_versions.contains(&version),
            });
        }

        sleep(Duration::from_millis(50)).await;
    }
}

pub async fn fetch_fabric_versions() -> HashSet<String> {
    let Ok(resp) = reqwest::get("https://meta.fabricmc.net/v2/versions/game")
        .await else {
            return HashSet::new();
        };

    let Ok(list) = resp.json::<Vec<serde_json::Value>>().await else {
        return HashSet::new();
    };

    list.iter()
        .filter_map(|v| v["version"].as_str().map(String::from))
        .collect()
}

pub async fn fetch_forge_versions() -> HashSet<String> {
    let Ok(resp) = reqwest::get("https://maven.minecraftforge.net/net/minecraftforge/forge/maven-metadata.xml")
        .await else {
            return HashSet::new();
        };

    let Ok(text) = resp.text().await else { 
        return HashSet::new();
    };

    text.lines()
        .filter_map(|line| {
            // extract mc version prefix: 1.21.1-xx.x.x
            line.split('-').next().map(String::from)
        })
        .collect()
}
