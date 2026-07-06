use reqwest::Client;
use serde::{Serialize,Deserialize};
use crate::AppState;

#[derive(Serialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse{
    pub token: String
}

pub struct ApiClient{
    pub(crate) base_url: String,
    pub(crate) client: Client,
}


impl ApiClient {
    pub fn new(base_url: String) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Self { base_url, client })
    }

    pub(crate) async fn check_status(response: reqwest::Response) -> anyhow::Result<reqwest::Response>{
        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_else(|_| "No error body".to_string());
            anyhow::bail!("HTTP {}: {}", status, error_body);
        }
        Ok(response)
    }

    pub async fn login(&self, email: &str, password: &str) -> anyhow::Result<String> {
        let url = format!("{}/api/login", self.base_url);
        let login_data = LoginRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let response = self.client.post(&url).json(&login_data).send().await?;
        let response = Self::check_status(response).await?;
        let auth: AuthResponse = response.json().await?;
        Ok(auth.token)
    }
}

#[tauri::command]
pub async fn login(
    state: tauri::State<'_, AppState>, email: String, password: String) -> Result<String, String> {
        state.client.login(&email,&password).await.map_err(|e| e.to_string())
}
