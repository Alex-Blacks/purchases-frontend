use serde::{Serialize,Deserialize};
use crate::api::ApiClient;
use crate::AppState;

#[derive(Serialize)]
pub struct CreateUserRequest{
    pub name: String,
    pub password: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct UserResponse {
	pub id: u64, 
	pub name: String,
	pub email: String,
	pub role: String,
	pub status: String,
}

#[derive(Debug,Deserialize, Serialize, Clone)]
pub struct UpdateUserRequest {
    #[serde(default)] 
	pub name: Option<String>,
    #[serde(default)] 
    pub password: Option<String>,
    #[serde(default)] 
	pub email: Option<String>,
    #[serde(default)] 
	pub role: Option<String>,
    #[serde(default)] 
	pub status: Option<String>,
}

impl ApiClient {
    pub async fn create_user(&self, name: &str, password: &str, email: &str, role: &str) -> anyhow::Result<UserResponse> {
        let url = format!("{}/api/users", self.base_url);
        let user_data = CreateUserRequest {
            name: name.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            role: role.to_string(),
        };

        let response = self.client.post(&url).json(&user_data).send().await?;
        let response = Self::check_status(response).await?;
        let user: UserResponse = response.json().await?;
        Ok(user)
    }

    pub async fn get_user(&self, token: &str, id: u64) -> anyhow::Result<UserResponse> {
        let url = format!("{}/api/private/users/{}", self.base_url, id);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let user: UserResponse = response.json().await?;
        Ok(user)
    }

    pub async fn delete_user(&self, token: &str, id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/users/{}", self.base_url, id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;
        Ok("User deleted successfully".to_string())
    }

    pub async fn update_user(&self, token: &str, id: u64, update: &UpdateUserRequest) -> anyhow::Result<UserResponse> {
        let url = format!("{}/api/private/users/{}", self.base_url, id);

        let response = self.client.patch(&url).json(update).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let user: UserResponse = response.json().await?;
        Ok(user)
    }

    pub async fn list_users(&self, token: &str) -> anyhow::Result<Vec<UserResponse>> {
        let url = format!("{}/api/private/users", self.base_url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let user: Vec<UserResponse> = response.json().await?;
        Ok(user)
    }
}

#[tauri::command]
pub async fn create_user(state: tauri::State<'_, AppState>, name: String, password: String, email: String, role: String) -> Result<UserResponse, String> {
    state.client.create_user(&name,&password,&email,&role).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_user(state: tauri::State<'_, AppState>, token: String, id: u64) -> Result<UserResponse, String> {
    state.client.get_user(&token, id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_user(state: tauri::State<'_, AppState>, token: String, id: u64) -> Result<String, String> {
    state.client.delete_user(&token, id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_user(state: tauri::State<'_, AppState>, token: String, id: u64, update: UpdateUserRequest) -> Result<UserResponse, String> {
    state.client.update_user(&token, id, &update).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_users(state: tauri::State<'_, AppState>, token: String) -> Result<Vec<UserResponse>, String> {
    state.client.list_users(&token).await.map_err(|e| e.to_string())
}