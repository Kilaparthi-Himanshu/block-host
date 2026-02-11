use std::{
    fs,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

use reqwest::Client;
use zip::ZipArchive;

/// Playit Binary Resolution
fn playit_binary_name() -> &'static str {
    #[cfg(target_os = "windows")]
    { "playit.exe" }

    #[cfg(not(target_os = "windows"))]
    { "playit" }
}


pub fn playit_binary(base: &PathBuf) -> PathBuf {
    base.join(playit_binary_name())
}

/// Installation Helpers
fn installing_marker(base: &PathBuf) -> PathBuf {
    base.join(".installing")
}

fn zip_path(base: &PathBuf) -> PathBuf {
    base.join("playit.zip")
}

/// Download URLs (official)
fn playit_download_url() -> &'static str {
    #[cfg(target_os = "windows")]
    { "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-windows-x86_64.exe" }

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    { "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-macos-x86_64" }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    { "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-macos-aarch64" }

    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    { "https://github.com/playit-cloud/playit-agent/releases/latest/download/playit-linux-x86_64" }
} 

/// Install Playit (transactional)
pub async fn install_playit(base: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(base).map_err(|e| e.to_string())?;

    let marker = installing_marker(base);
    fs::write(&marker, b"").ok();

    let resp = Client::new()
        .get(playit_download_url())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Failed to download playit: {}", resp.status()));
    }

    let bytes = resp.bytes().await.map_err(|e| e.to_string())?;

    if bytes.len() < 500_000 {
        return Err("Downloaded playit binary is suspiciously small".into());
    }

    let bin = playit_binary(base);
    fs::write(&bin, &bytes).map_err(|e| e.to_string())?;

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).ok();
    }

    fs::remove_file(&marker).ok();
    Ok(())
}

/// Verification

fn playit_runs(bin: &Path) -> bool {
    Command::new(bin)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Cleanup (nuke strategy)
fn cleanup_playit(base: &PathBuf) {
    fs::remove_dir_all(base).ok();
}

/// Installed Check
pub fn playit_installed(base: &PathBuf) -> bool {
    let bin = playit_binary(base);

    if installing_marker(base).exists() {
        cleanup_playit(base);
        return false;
    }

    if !bin.exists() {
        cleanup_playit(base);
        return false;
    }

    if !playit_runs(&bin) {
        cleanup_playit(base);
        return false;
    }

    true
}

pub fn start_playit(base: &PathBuf) -> Result<Child, String> {
    let bin = playit_binary(base);

    let child = Command::new(bin)
        .arg("agent")
        .current_dir(base)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(child)
}
