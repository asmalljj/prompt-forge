#[tauri::command]
pub fn hello(name: String) -> String {
    format!("hello,{}!", name)
}