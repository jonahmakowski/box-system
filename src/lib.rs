mod database;
mod qrcode_gen;

// Frontend
pub mod cli_mode;
pub mod webui;

// Globals
const DB_FILE: &str = "./db_file.json";
const CODES_FOLDER: &str = "./codes/";
