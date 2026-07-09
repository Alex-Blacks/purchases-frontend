mod stores;
mod categories;
mod products;
mod orders;
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
            users::get_user,
            users::delete_user,
            users::update_user,
            users::list_users,
            stores::create_store,
            stores::get_store,
            stores::delete_store,
            stores::list_stores,
            categories::create_category,
            categories::get_category,
            categories::delete_category,
            categories::list_categories,
            products::create_product,
            products::get_product,
            products::delete_product,
            products::list_products,
            products::find_product_by_alias,
            products::create_product_alias,
            products::get_product_alias,
            products::delete_product_alias,
            products::delete_all_product_aliases,
            products::list_product_aliases,
            orders::create_order,
            orders::get_order,
            orders::delete_order,
            orders::list_orders,
            orders::add_item,
            orders::update_item,
            orders::delete_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}