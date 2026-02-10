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
    { "https://playit.gg/downloads/playit-windows-x86_64.zip" }

    #[cfg(target_os = "macos")]
    { "https://playit.gg/downloads/playit-macos-x86_64.zip" }

    #[cfg(target_os = "linux")]
    { "https://playit.gg/downloads/playit-linux-x86_64.zip" }
}

/// Install Playit (transactional)
pub async fn install_playit(base: &PathBuf) -> Result<(), String> {
    fs::create_dir_all(base).map_err(|e| e.to_string())?;

    let marker = installing_marker(base);
    fs::write(&marker, b"").ok();

    let bytes = Client::new()
        .get(playit_download_url())
        .send()
        .await
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    let zip = zip_path(base);
    fs::write(&zip, &bytes).map_err(|e| e.to_string())?;

    let file = fs::File::open(&zip).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = base.join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath).ok();
        } else {
            let mut out = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut out).map_err(|e| e.to_string())?;
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::PermissionsExt;
        let bin = playit_binary(base);
        fs::set_permissions(&bin, fs::Permissions::from_mode(0o755)).ok();
    }

    fs::remove_file(&marker).ok();
    fs::remove_file(&zip).ok();

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
