use std::{f32::consts::E, fs, path::PathBuf, process::Command};

use serde::{Deserialize, Serialize};

use crate::{commands::server_creation::LoaderType, state::app_state::AppState, utils::path::servers_dir};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub version: String,
    pub loader: LoaderType,
    pub ram_gb: u8,
    pub path: String,
    pub created_at: i64,
}

#[tauri::command]
pub fn list_servers() -> Result<Vec<ServerConfig>, String> {
    let base = servers_dir();
    let mut servers = Vec::new();

    // First install: no servers directory yet
    if !base.exists() {
        return Ok(servers);
    }

    for version_dir in fs::read_dir(base).map_err(|e| e.to_string())? {
        let version_dir = version_dir.map_err(|e| e.to_string())?;
        if !version_dir.path().is_dir() { continue; } // continue if its not a folder and is a file

        for server_dir in fs::read_dir(version_dir.path()).map_err(|e| e.to_string())? {
            let server_dir = server_dir.map_err(|e| e.to_string())?;
            let config_path = server_dir.path().join("cubely.json");

            if config_path.exists() {
                let content = fs::read_to_string(config_path)
                    .map_err(|e| e.to_string())?;
                let config: ServerConfig = serde_json::from_str(&content)
                    .map_err(|e| e.to_string())?;
                servers.push(config);
            }
        }
    }

    Ok(servers)
}

use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct ServerProperties {
    pub motd: String,
    pub online_mode: bool,
    pub max_players: u32,
    pub difficulty: String,
    pub gamemode: String,
    pub pvp: bool,
    pub spawn_protection: u32,
    pub view_distance: u32,
    pub simulation_distance: u32,
    pub server_port: u16
}

pub fn map_server_properties(server_path: &String) -> Result<HashMap<String, String>, String> {
    let content = fs::read_to_string(
        PathBuf::from(server_path).join("server.properties")
    ).map_err(|e| e.to_string())?;

    let mut map = std::collections::HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { 
            continue; 
        }

        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.to_string(), v.to_string());
        }
    }

    Ok(map)
}

#[tauri::command]
pub fn read_server_properties(server_path: String) -> Result<ServerProperties, String> {
    let map = map_server_properties(&server_path)?;

    Ok(ServerProperties {
        motd: map.get("motd").cloned().unwrap_or_default(),
        online_mode: map.get("online-mode").map(|v| v == "true").unwrap_or(true),
        max_players: map.get("max-players").and_then(|v| v.parse().ok()).unwrap_or(20),
        difficulty: map.get("difficulty").cloned().unwrap_or("easy".into()),
        gamemode: map.get("gamemode").cloned().unwrap_or("survival".into()),
        pvp: map.get("pvp").map(|v| v == "true").unwrap_or(true),
        spawn_protection: map.get("spawn-protection").and_then(|v| v.parse().ok()).unwrap_or(16),
        view_distance: map.get("view-distance").and_then(|v| v.parse().ok()).unwrap_or(10),
        simulation_distance: map.get("simulation-distance").and_then(|v| v.parse().ok()).unwrap_or(10),
        server_port: map.get("server-port").and_then(|v| v.parse().ok()).unwrap_or(25565)
    })
}

#[tauri::command]
pub async fn write_server_properties(
    server_path: String,
    props: ServerProperties
) -> Result<(), String> {
    let mut map = map_server_properties(&server_path)?;

    // Update only keys we control
    map.insert("motd".into(), props.motd);
    map.insert("online-mode".into(), props.online_mode.to_string());
    map.insert("max-players".into(), props.max_players.to_string());
    map.insert("difficulty".into(), props.difficulty);
    map.insert("gamemode".into(), props.gamemode);
    map.insert("pvp".into(), props.pvp.to_string());
    map.insert("spawn-protection".into(), props.spawn_protection.to_string());
    map.insert("view-distance".into(), props.view_distance.to_string());
    map.insert("simulation-distance".into(), props.simulation_distance.to_string());
    map.insert("server-port".into(), props.server_port.to_string());

    //  Write back EVERYTHING (including unknown keys)
    let mut output = String::from("# Generated / Updated by Cubely\n");

    for (k, v) in map.iter() {
        output.push_str(&format!("{k}={v}\n"));
    }

    fs::write(&PathBuf::from(server_path).join("server.properties"), output).map_err(|e| e.to_string())?;

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveServer {
    pub server_id: String,
    pub mc_pid: u32,
    pub ngrok_pid: u32,
    pub public_url: String,
}

#[tauri::command]
pub fn get_active_server(
    state: tauri::State<'_, AppState>,
) -> Option<ActiveServer> {
    state.active_server.lock().unwrap().clone()
}

#[tauri::command]
pub async fn start_server(
    server: ServerConfig,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut active = state.active_server.lock().unwrap();

    if active.is_some() {
        return Err("A server is already running".into());
    }

    // spawn minecraft
    let mc_child = Command::new("java")
        .args([
            format!("-Xmx{}G", server.ram_gb),
            format!("-Xms{}G", server.ram_gb),
            "-jar".into(),
            "server.jar".into(),
            "nogui".into(),
        ])
        .current_dir(&server.path)
        .spawn()
        .map_err(|e| e.to_string())?;

    // spawn ngrok
    let ngrok_child = Command::new("java")
        .args(["tcp", "localhost:25565"])
        .spawn()
        .map_err(|e| e.to_string())?;

    *active = Some(ActiveServer { 
        server_id: server.id.clone(), 
        mc_pid: mc_child.id(), 
        ngrok_pid: ngrok_child.id(), 
        public_url: "pending...".into() // fill after API fetch
    });

    Ok(())
}

#[tauri::command]
pub fn stop_server(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut active = state.active_server.lock().unwrap();

    if let Some(server) = active.take() {
        let _ = Command::new("kill").arg(server.mc_pid.to_string()).status();
        let _ = Command::new("kill").arg(server.ngrok_pid.to_string()).status();
        Ok(())
    } else {
        Err("No active server".into())
    }
}
