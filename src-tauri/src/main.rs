// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{self, Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    sync::Mutex,
};
use tauri::api::dialog::blocking::FileDialogBuilder;

#[derive(Default)]
pub struct AppState(Mutex<App>);

#[derive(Default)]
pub struct App {
    backup_file: BackupFile,
}

impl AppState {
    pub fn reset(&mut self) {}
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![load_command, save_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_command(state: tauri::State<AppState>) -> Result<String,String> {
    let dialog_result = FileDialogBuilder::new()
        .add_filter("bachys", &["bach"])
        .pick_file();
    match dialog_result {
        None => Err("Could not open file".into()),
        Some(path) => {
            let file_contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let backup: BackupFile = serde_json::from_str(&file_contents).map_err(|e| e.to_string())?;
            state.0.lock().unwrap().backup_file = backup;

            Ok(file_contents)
        }
    }
}

#[tauri::command]
fn save_command(backup_config : &str) -> Result<String,String> {
    let backup = mock_backupfile();
    // check if data is parsable, else throw error
    let _:BackupFile = serde_json:: from_str(backup_config).map_err(|e| e.to_string())?;
    let serialized = serde_json::to_string(&backup).map_err(|e| e.to_string())?;

    let dialog_result = FileDialogBuilder::new()
        .add_filter("Bachys", &["bach"])
        .save_file();
    match dialog_result {
        None => Err("Did NOT save the file!".to_string()),
        Some(path) => {
            let mut file = fs::OpenOptions::new().write(true).open(&path).map_err(|e| e.to_string())?;
            file.write_all(serialized.as_bytes()).map_err(|e| e.to_string())?;
            Ok(format!("Saved to file \"{}\"", &path.as_path().display()))
        }
    }
}

fn mock_backupfile() -> BackupFile {
    let bachy1 = Bachy {
        id: 0,
        name: String::from("Bachy1"),
        icon: String::from("🧪"),
        target: PathBuf::from("/path/to/target1"),
        files: vec![
            FileInfo {
                path: PathBuf::from("/path/to/file3.txt"),
                last_backup: String::from("Content3"),
            },
            FileInfo {
                path: PathBuf::from("/path/to/file3.txt"),
                last_backup: String::from("Content3"),
            },
        ],
    };

    let bachy2 = Bachy {
        id: 1,
        name: String::from("Bachy2"),
        icon: String::from("🧪"),
        target: PathBuf::from("/path/to/target2"),
        files: vec![
            FileInfo {
                path: PathBuf::from("/path/to/file3.txt"),
                last_backup: String::from("Content3"),
            },
            FileInfo {
                path: PathBuf::from("/path/to/file3.txt"),
                last_backup: String::from("Content3"),
            },
        ],
    };

    let bachy_vec = vec![bachy1,bachy2];

    BackupFile {
        name: String::from("MeinBackup"),
        paht: PathBuf::from("/path/to/file/MeinBackup.bach"),
        bachys: bachy_vec,
    }
}

/* #[tauri::command]
fn doBackup() {}

#[tauri::command]
fn addBachy() {}

#[tauri::command]
fn removeBachy(id: i32) {} */

#[derive(Serialize, Deserialize, Debug, Default)]
struct Bachy {
    id: i32,
    name: String,
    icon: String,
    target: PathBuf,
    files: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct BackupFile {
    name: String,
    paht: PathBuf,
    bachys: Vec<Bachy>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct FileInfo{
    path: PathBuf, 
    last_backup: String
}