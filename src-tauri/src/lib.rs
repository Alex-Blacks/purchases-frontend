use reqwest::Client;
use serde::{Serialize,Deserialize};

#[derive(Serialize)]
pub struct CreateUserRequest{
    pub name: String,
    pub password: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug,Deserialize, Serialize)]
pub struct UserResponse {
	pub id: u64, 
	pub name: String,
	pub email: String,
	pub role: String,
	pub status: String,
}
#[derive(Serialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse{
    pub token: String
}

#[derive(Serialize)]
pub struct CreateStoreRequest{
    pub name: String
}

#[derive(Serialize)]
pub struct GetStoreRequest{
    pub id: u64
}

#[derive(Serialize)]
pub struct DeleteStoreRequest{
    pub id: u64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreResponse{
    pub id: u64,
    pub name: String
}

pub struct ApiClient{
    base_url: String,
    client: Client,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn create_user(
        &self,
        name: &str,
        password: &str,
        email: &str,
        role: &str,
    ) -> anyhow::Result<UserResponse> {
        let url = format!("{}/api/users", self.base_url);
        let user_data = CreateUserRequest {
            name: name.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            role: role.to_string(),
        };

        let response = self.client
            .post(&url)
            .json(&user_data)
            .send()
            .await?;

        // Проверяем статус вручную
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "No error body".to_string());
            anyhow::bail!("HTTP {}: {}", status, error_body);
        }

        let user: UserResponse = response.json().await?;
        Ok(user)
    }

    pub async fn login(&self, email: &str, password: &str) -> anyhow::Result<String> {
        let url = format!("{}/api/login", self.base_url);
        let login_data = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self.client
            .post(&url)
            .json(&login_data)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "No error body".to_string());
            anyhow::bail!("HTTP {}: {}", status, error_body);
        }

        let auth: AuthResponse = response.json().await?;
        Ok(auth.token)
    }

    pub async fn create_store(&self, token: &str, name: String) -> anyhow::Result<StoreResponse> {
        let url = format!("{}/api/private/stores", self.base_url);
        let store_data = CreateStoreRequest { name };

        let response = self.client
            .post(&url)
            .json(&store_data)
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "No error body".to_string());
            anyhow::bail!("HTTP {}: {}", status, error_body);
        }

        let store: StoreResponse = response.json().await?;
        Ok(store)
    }

    pub async fn get_store(&self, token: &str, id: u64) -> anyhow::Result<StoreResponse> {
        let url = format!("{}/api/private/stores", self.base_url);
        let store_data = GetStoreRequest { id };

        let response = self.client
            .post(&url)
            .json(&store_data)
            .bearer_auth(token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "No error body".to_string());
            anyhow::bail!("HTTP {}: {}", status, error_body);
        }

        let store: StoreResponse = response.json().await?;
        Ok(store)
    }
}
pub struct AppState {
    client: ApiClient,
}

#[tauri::command]
async fn login(
    state: tauri::State<'_, AppState>,
    email: String,
    password: String,
) -> Result<String, String> {
    state
        .client
        .login(&email,&password)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_user(
    state: tauri::State<'_, AppState>,
    name: String,
    password: String,
    email: String,
    role: String,
) -> Result<UserResponse, String> {
    state
        .client
        .create_user(&name,&password,&email,&role)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_store(
    state: tauri::State<'_, AppState>,
    token: String,
    name: String,
) -> Result<StoreResponse, String> {
    state
        .client
        .create_store(&token, name)
        .await
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let client = ApiClient::new("http://localhost:8080".to_string());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {client})
        .invoke_handler(tauri::generate_handler![
            login,
            create_user,
            create_store,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
