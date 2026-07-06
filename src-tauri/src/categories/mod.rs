use serde::{Deserialize,Serialize};
use crate::api::ApiClient;
use crate::AppState;

#[derive(Serialize)]
pub struct CreateCategoryRequest{
    pub name: String
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct CategoryResponse{
    pub id: u64,
    pub name: String
}

impl ApiClient {
    pub async fn create_category(&self, token: &str, name: String) -> anyhow::Result<CategoryResponse> {
        let url = format!("{}/api/private/categories", self.base_url);
        let category_data = CreateCategoryRequest { name };

        let response = self.client.post(&url).json(&category_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let category: CategoryResponse = response.json().await?;
        Ok(category)
    }

    pub async fn get_category(&self, token: &str, id: u64) -> anyhow::Result<CategoryResponse> {
        let url = format!("{}/api/private/categories/{}", self.base_url, id);
        
        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let category: CategoryResponse = response.json().await?;
        Ok(category)
    }

    pub async fn delete_category(&self, token: &str, id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/categories/{}", self.base_url, id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;

        Ok("Category deleted successfully".to_string())
    }

    pub async fn list_category(&self, token: &str) -> anyhow::Result<Vec<CategoryResponse>> {
        let url = format!("{}/api/private/categories", self.base_url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let categories: Vec<CategoryResponse> = response.json().await?;
        Ok(categories)
    }
}


#[tauri::command]
pub async fn create_category(state: tauri::State<'_,AppState>, token: String, name: String) -> Result<CategoryResponse, String> {
    state.client.create_category(&token, name).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn get_category(state: tauri::State<'_,AppState>,token: String,id:u64) -> Result<CategoryResponse,String> {
    state.client.get_category(&token, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_category(state: tauri::State<'_,AppState>, token: String, id:u64) -> Result<String, String> {
    state.client.delete_category(&token, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn list_categories(state: tauri::State<'_,AppState>, token: String) -> Result<Vec<CategoryResponse>, String> {
    state.client.list_category(&token).await.map_err(|e|e.to_string())
}