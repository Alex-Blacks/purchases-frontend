mod stores;
mod categories;
mod users;
mod api;
use api::ApiClient;


pub struct AppState {
    pub client: ApiClient,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = ApiClient::new("http://localhost:8080".to_string()).expect("Failed to create API client");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {client})
        .invoke_handler(tauri::generate_handler![
            api::login,
            users::create_user,
            stores::create_store,
            stores::get_store,
            stores::delete_store,
            stores::list_stores,
            categories::create_category,
            categories::get_category,
            categories::delete_category,
            categories::list_categories,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}