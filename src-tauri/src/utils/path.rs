use std::path::PathBuf;

pub fn servers_dir() -> PathBuf {
    let mut dir = dirs::data_dir().expect("Failed to get data dir");
    dir.push("BlockHost");
    dir.push("servers");
    dir

    // This resolves to:
    // Windows → C:\Users\<you>\AppData\Roaming\BlockHost\servers
    // macOS → ~/Library/Application Support/BlockHost/servers (i.e. /Users/<your-username>/Library/Application Support/BlockHost/servers/)
    // Linux → ~/.local/share/BlockHost/servers
}
