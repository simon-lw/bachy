// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{self, Deserialize, Serialize};
use std::{
    fs::{self},
    io::{self, Write},
    path::{Path, PathBuf},
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
        .invoke_handler(tauri::generate_handler![
            load_command,
            save_command,
            get_default_command,
            do_backup_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn load_command(state: tauri::State<'_, AppState>) -> Result<String, String> {
    println!("Called load");
    let dialog_result = FileDialogBuilder::new()
        .set_directory(".")
        .add_filter("bachys", &["bach"])
        .pick_file();
    
    match dialog_result {
        None => Err("Could not open file".into()),
        Some(path) => {
            let file_contents = fs::read_to_string(path).map_err(|e| e.to_string())?;
            let backup: BackupFile =
                serde_json::from_str(&file_contents).map_err(|e| e.to_string())?;
            state.0.lock().unwrap().backup_file = backup;

            Ok(file_contents)
        }
    }
}

#[tauri::command]
fn get_default_command() -> Result<String, String> {
    let default_backup = BackupFile::default();
    let backup_json = serde_json::to_string(&default_backup).map_err(|e| e.to_string())?;

    Ok(backup_json)
}

#[tauri::command]
async fn save_command(config: &str) -> Result<String, String> {
    let bachupfile: BackupFile = serde_json::from_str(config).map_err(|e| e.to_string())?;

    let dialog_result = FileDialogBuilder::new()
        .set_directory(".")
        .add_filter("Bachys", &["bach"])
        .set_file_name(&bachupfile.name)
        .save_file();
    match dialog_result {
        None => Err("Did NOT save the file!".to_string()),
        Some(path) => {
            let mut file = fs::File::create(&path).map_err(|e| e.to_string())?;

            file.write_all(config.as_bytes())
                .map_err(|e| e.to_string())?;
            Ok(format!("Saved to file \"{}\"", &path.as_path().display()))
        }
    }
}

#[tauri::command]
fn do_backup_command(config: &str) -> Result<String, String> {
    let mut bachy: Bachy = serde_json::from_str(config).map_err(|e| e.to_string())?;
    let target = &bachy.target;

    if !target.exists() {
        return Err(format!("The target:{} does not exist!", target.display()));
    }

    for file_info in &mut bachy.files {
        if file_info.path.is_file() {
            let rel_name = PathBuf::from(
                file_info
                    .path
                    .file_name()
                    .ok_or(format!(
                        "Could not get Filename from Path \"{}\"",
                        file_info.path.display()
                    ))
                    .map_err(|e| e.to_string())?,
            );

            let file_target = target.join(rel_name);
            fs::copy(&file_info.path, file_target).map_err(|e| e.to_string())?;

            file_info.last_backup = chrono::Local::now().timestamp().to_string();
        } else {
            let mut prefix = file_info.path.clone();

            if !prefix.pop() {
                return Err(format!(
                    "Could not get Parent from \"{}\"",
                    prefix.display()
                ));
            }

            let (folders, files) =
                find_files_to_copy(&file_info.path).map_err(|e| e.to_string())?;

            // copy all folders first
            for folder in &folders {
                let rel_target = folder.strip_prefix(&prefix).map_err(|e| e.to_string())?;
                let folder_target = target.join(rel_target);

                if !folder_target.exists() {
                    fs::create_dir(folder_target).map_err(|e| e.to_string())?;
                }
            }

            // files
            for f in &files {
                let rel_name = f.strip_prefix(&prefix).map_err(|e| e.to_string())?;
                let file_target = target.join(rel_name);
                fs::copy(f, file_target).map_err(|e| e.to_string())?;
            }

            file_info.last_backup = chrono::Local::now().timestamp().to_string();
        }
    }

    serde_json::to_string(&bachy).map_err(|e| e.to_string())
}

/// Finds all files and folders within a folder.
/// If a folder is passed in, the folder is also added to the list of dirs
///
/// Returns a tupel of (dirEntries, files)
fn find_files_to_copy(source: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), io::Error> {
    let mut vec_files: Vec<PathBuf> = Vec::new();
    let mut vec_folders: Vec<PathBuf> = Vec::new();
    let mut vec_to_handle: Vec<PathBuf> = Vec::new();

    if source.is_dir() {
        vec_to_handle.push(source.to_path_buf());
        vec_folders.push(source.to_path_buf());
    } else {
        vec_files.push(source.to_path_buf());
    }

    while !vec_to_handle.is_empty() {
        if let Some(next_source) = vec_to_handle.pop() {
            if let Ok(entries) = fs::read_dir(next_source) {
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            vec_files.push(entry.path().to_path_buf());
                        } else if metadata.is_dir() {
                            vec_folders.push(entry.path().to_path_buf());
                            vec_to_handle.push(entry.path().to_path_buf());
                        }
                    }
                }
            }
        }
    }

    Ok((vec_folders, vec_files))
}

/// Copies all files in [files] relative to target
// fn copy_if_new(files: Vec<PathBuf>, target: PathBuf, source: PathBuf) {
//     for file in files {
//         let rel_target = file.strip_prefix(&source).unwrap();
//         let target = target.join(rel_target);

//         let target_time = target.metadata().unwrap().modified().unwrap();
//         let source_time = file.metadata().unwrap().modified().unwrap();

//         if target_time < source_time {
//             fs::copy(file, target);
//         }
//     }
// }

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
struct FileInfo {
    path: PathBuf,
    last_backup: String,
}
