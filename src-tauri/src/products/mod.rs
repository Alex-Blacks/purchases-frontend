use serde::{Deserialize,Serialize};
use crate::api::ApiClient;
use crate::AppState;


#[derive(Serialize)]
pub struct CreateProductRequest {
	pub title: String,
    pub unit: String,
	pub category_id: u64
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct ProductResponse {
	pub id: u64,
    pub title: String, 
	pub unit: String, 
	pub category: String 
}

#[derive(Serialize)]
pub struct CreateProductAliasRequest {
	pub alias: String
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct CreateProductAliasResponse {
	pub alias_id: u64
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct ProductFindResponse {
	pub product: String
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct ProductAliasResponse {
	pub id: u64,
	pub product: String,
	pub alias: String
}

impl ApiClient {
    pub async fn create_product(&self, token: &str, title: String, unit: String, category_id: u64) -> anyhow::Result<ProductResponse> {
        let url = format!("{}/api/private/products", self.base_url);
        let product_data = CreateProductRequest{ title, unit, category_id};

        let response = self.client.post(&url).json(&product_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let product: ProductResponse = response.json().await?;
        Ok(product)
    }

    pub async fn get_product(&self, token: &str, id: u64) -> anyhow::Result<ProductResponse> {
        let url = format!("{}/api/private/products/{}", self.base_url, id);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let product: ProductResponse = response.json().await?;
        Ok(product)
    }
    pub async fn delete_product(&self, token: &str, id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/products/{}", self.base_url, id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;

        Ok("Product deleted successfully".to_string())
    }
    pub async fn list_products(&self, token: &str) -> anyhow::Result<Vec<ProductResponse>> {
        let url = format!("{}/api/private/products", self.base_url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let products: Vec<ProductResponse> = response.json().await?;
        Ok(products)
    }
    
    // PRODUCT ALIAS

    pub async fn create_product_alias(&self, token: &str, product_id: u64, alias: String) -> anyhow::Result<CreateProductAliasResponse> {
        let url = format!("{}/api/private/products/{}/aliases", self.base_url, product_id);
        let product_alias_data = CreateProductAliasRequest{ alias };

        let response = self.client.post(&url).json(&product_alias_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let product_alias: CreateProductAliasResponse = response.json().await?;
        Ok(product_alias)
    }

    pub async fn get_product_alias(&self, token: &str, product_id: u64, id: u64) -> anyhow::Result<ProductAliasResponse> {
        let url = format!("{}/api/private/products/{}/aliases/{}", self.base_url, product_id, id);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let product_alias: ProductAliasResponse = response.json().await?;
        Ok(product_alias)
    }
    pub async fn delete_product_alias(&self, token: &str, product_id: u64, id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/products/{}/aliases/{}", self.base_url, product_id ,id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;

        Ok("Product alias deleted successfully".to_string())
    }
    pub async fn delete_all_product_aliases(&self, token: &str, product_id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/products/{}/aliases", self.base_url, product_id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;

        Ok("All product aliases deleted successfully".to_string())
    }
    pub async fn list_product_aliases(&self, token: &str, product_id: u64) -> anyhow::Result<Vec<ProductAliasResponse>> {
        let url = format!("{}/api/private/products/{}/aliases", self.base_url, product_id);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let products_aliases: Vec<ProductAliasResponse> = response.json().await?;
        Ok(products_aliases)
    }
    pub async fn find_product_by_alias(&self, token: &str, alias: String) -> anyhow::Result<ProductFindResponse> {
        let url = format!("{}/api/private/products/by-alias", self.base_url);

        let response = self.client.get(&url).query(&alias).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let product: ProductFindResponse = response.json().await?;
        Ok(product)
    }
}

#[tauri::command]
pub async fn create_product(state: tauri::State<'_, AppState>, token: String, title: String, unit: String, category_id: u64) -> Result<ProductResponse, String> {
    state.client.create_product(&token, title, unit, category_id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn get_product(state: tauri::State<'_,AppState>, token: String, id: u64) -> Result<ProductResponse, String> {
    state.client.get_product(&token, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_product(state: tauri::State<'_,AppState>, token: String, id: u64) -> Result<String, String> {
    state.client.delete_category(&token, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn list_products(state: tauri::State<'_,AppState>, token: String) -> Result<Vec<ProductResponse>, String> {
    state.client.list_products(&token).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn create_product_alias(state: tauri::State<'_, AppState>, token: String, product_id: u64, alias: String) -> Result<CreateProductAliasResponse, String> {
    state.client.create_product_alias(&token, product_id, alias).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn get_product_alias(state: tauri::State<'_,AppState>, token: String, product_id: u64, id: u64) -> Result<ProductAliasResponse, String> {
    state.client.get_product_alias(&token, product_id, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_product_alias(state: tauri::State<'_,AppState>, token: String, product_id: u64, id: u64) -> Result<String, String> {
    state.client.delete_product_alias(&token, product_id, id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_all_product_aliases(state: tauri::State<'_,AppState>, token: String, product_id: u64) -> Result<String, String> {
    state.client.delete_all_product_aliases(&token, product_id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn list_product_aliases(state: tauri::State<'_,AppState>, token: String, product_id: u64) -> Result<Vec<ProductAliasResponse>, String> {
    state.client.list_product_aliases(&token, product_id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn find_product_by_alias(state: tauri::State<'_,AppState>, token: String, alias: String) -> Result<ProductFindResponse, String> {
    state.client.find_product_by_alias(&token, alias).await.map_err(|e|e.to_string())
}
