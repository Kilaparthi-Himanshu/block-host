// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

mod state;
mod commands;
mod utils;

fn main() {
    let path = utils::path::servers_dir();
    // println!("GGWP: {:?}", path);
    app_lib::run();
}
