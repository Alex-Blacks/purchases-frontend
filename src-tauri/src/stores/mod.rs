use serde::{Deserialize, Serialize};
use crate::api::ApiClient;
use crate::AppState;

#[derive(Serialize)]
pub struct CreateStoreRequest{
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StoreResponse{
    pub id: u64,
    pub name: String
}

impl ApiClient {
    pub async fn create_store(&self, token: &str, name: String) -> anyhow::Result<StoreResponse> {
        let url = format!("{}/api/private/stores", self.base_url);
        let store_data = CreateStoreRequest { name };

        let response = self.client.post(&url).json(&store_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let store: StoreResponse = response.json().await?;
        Ok(store)
    }

    pub async fn get_store(&self, token: &str, id: u64) -> anyhow::Result<StoreResponse> {
        let url = format!("{}/api/private/stores/{}", self.base_url, id);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let store: StoreResponse = response.json().await?;
        Ok(store)
    }

    pub async fn delete_store(&self, token: &str, id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/stores/{}", self.base_url, id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;

        Ok("Store deleted successfully".to_string())
    }

    pub async fn list_stores(&self, token: &str) -> anyhow::Result<Vec<StoreResponse>> {
        let url = format!("{}/api/private/stores", self.base_url);
        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let stores: Vec<StoreResponse> = response.json().await?;

        Ok(stores)
    }
}

#[tauri::command]
pub async fn create_store(state: tauri::State<'_, AppState>, token: String, name: String) -> Result<StoreResponse, String> {
    state.client.create_store(&token, name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_store(state: tauri::State<'_, AppState>, token: String, id: u64) -> Result<StoreResponse, String> {
    state.client.get_store(&token, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_store(state: tauri::State<'_, AppState>, token: String, id: u64) -> Result<String, String> {
    state.client.delete_store(&token, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn list_stores(state: tauri::State<'_, AppState>, token: String) -> Result<Vec<StoreResponse>, String> {
    state.client.list_stores(&token).await.map_err(|e| e.to_string())
}
