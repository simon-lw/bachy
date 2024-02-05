// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{self, Deserialize, Serialize};
use std::{default, error::Error, fmt::format, fs::{self, File}, io::Write, path::PathBuf, sync::Mutex};
use tauri::api::dialog::blocking::FileDialogBuilder;


#[derive(Default)]
pub struct AppState(Mutex<App>);

#[derive(Default)]
pub struct App{
    bachys : Vec<Bachy>
}

impl AppState{
    pub fn reset(&mut self) {

    }
}

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![load_command, save_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn load_command(state: tauri::State<AppState>) -> String{

    let dialog_result = FileDialogBuilder::new().add_filter("bachys", &["bach"]).pick_file();
    match dialog_result {
        None => "Could not open file".to_string(),
        Some(path) => {
            match get_bachies(&path) {

                Ok(bachys) => {
                    state.0.lock().unwrap().bachys = bachys;

                    String::new()
                },
                Err(_) => format!("Could not read/parse the file \"{}\"",path.as_path().display()),
            }
        }
    }
}

fn get_bachies( path : &PathBuf) -> Result<Vec<Bachy>, Box<dyn Error>>{
    let file_contents = fs::read_to_string(path)?;
    let bachys :Vec<Bachy>  = serde_json::from_str(&file_contents)?;
    Ok(bachys)
}

#[tauri::command]
fn save_command() -> String{
   let bachy1 = Bachy {
        name: String::from("Bachy1"),
        target: PathBuf::from("/path/to/target1"),
        files: vec![
            (PathBuf::from("/path/to/file1.txt"), String::from("Content1")),
            (PathBuf::from("/path/to/file2.txt"), String::from("Content2")),
        ],
    };

    let bachy2 = Bachy {
        name: String::from("Bachy2"),
        target: PathBuf::from("/path/to/target2"),
        files: vec![
            (PathBuf::from("/path/to/file3.txt"), String::from("Content3")),
            (PathBuf::from("/path/to/file4.txt"), String::from("Content4")),
        ],
    };

  let mut bachy_vec: Vec<Bachy> = Vec::new();
    bachy_vec.push(bachy1);
    bachy_vec.push(bachy2);


    let serialized = serde_json::to_string(&bachy_vec).unwrap();

    let dialog_result = FileDialogBuilder::new().add_filter("Bachys" , &["bach"]).save_file();
    match dialog_result {
        None => "Did NOT save the file!".to_string(),
        Some(path) => {
            let mut file = File::create(&path).unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
            format!("Saved to file \"{}\"",&path.as_path().display())

//                Err(_) => format!("Could not read/parse the file \"{}\"",path.as_path().display()),
        }
    }

}

#[tauri::command]
fn doBackup() {}

#[tauri::command]
fn addBachy() {}

#[tauri::command]
fn removeBachy(id: i32) {}

#[derive(Serialize, Deserialize, Debug)]
struct Bachy {
    name: String,
    target: PathBuf,
    files: Vec<(PathBuf, String)>,
}
