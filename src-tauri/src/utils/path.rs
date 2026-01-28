use std::path::PathBuf;

pub fn servers_dir() -> PathBuf {
    let mut dir = dirs::data_dir().expect("Failed to get data dir");
    dir.push("BlockHost");
    dir.push("servers");
    dir
}
