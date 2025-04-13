use chrono::Utc;
use db::SqliteDB;
use db::{CreateLecture, LectureData, LectureStore, Line};
use std::sync::Mutex;
use tauri::State;
use uuid::Uuid;

mod db;
#[tauri::command]
fn get_lectures(state: State<'_, Mutex<SqliteDB>>) -> Vec<CreateLecture> {
    let db = state.lock().unwrap();
    db.get_lectures()
}

#[tauri::command]
fn insert_lecture(state: State<'_, Mutex<SqliteDB>>, name: String) -> Result<(), String> {
    let db = state.lock().unwrap();

    let lid = Uuid::new_v4().to_string();
    let updated = Utc::now().timestamp() as u64;

    let lecture = CreateLecture { name, lid, updated };

    println!("{:?}", lecture);
    db.insert_lecture(lecture).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_lecture(state: State<'_, Mutex<SqliteDB>>, lid: String) -> Result<LectureData, String> {
    let db = state.lock().unwrap();
    db.get_lecture_struct(&lid).map_err(|e| e.to_string())
}

#[tauri::command]
fn insert_line(
    state: State<'_, Mutex<SqliteDB>>,
    lid: String,
    secid: u32,
    lineno: u32,
    data: String,
) -> Result<(), String> {
    let db = state.lock().unwrap();
    let line = Line {
        lid,
        secid,
        lineno,
        data,
    };
    db.insert_line(line).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_line(
    state: State<'_, Mutex<SqliteDB>>,
    lid: String,
    secid: u32,
    lineno: u32,
) -> Result<(), String> {
    let db = state.lock().unwrap();
    let line = Line {
        lid,
        secid,
        lineno,
        data: String::new(), // not used for deletion
    };
    db.delete_line(line).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = SqliteDB::new().expect("Failed to create DB");
    tauri::Builder::default()
        .manage(Mutex::new(db))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_lectures,
            insert_lecture,
            get_lecture,
            insert_line,
            delete_line
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
