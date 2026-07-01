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

#[derive(Debug,Deserialize, Serialize)]
pub struct UserResponse {
	pub id: u64, 
	pub name: String,
	pub email: String,
	pub role: String,
	pub status: String,
}

impl ApiClient {
    pub async fn create_user(&self, name: &str, password: &str, email: &str, role: &str,) -> anyhow::Result<UserResponse> {
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
}

#[tauri::command]
pub async fn create_user(
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