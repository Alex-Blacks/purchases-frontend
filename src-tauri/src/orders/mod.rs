use chrono::{DateTime,Utc};
use serde::{Deserialize,Serialize};
use crate::api::ApiClient;
use crate::AppState;

#[derive(Serialize)]
pub struct CreateOrderRequest {
    #[serde(rename = "storeId")]
	pub store_id: u64
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct OrderResponse {
	pub id: u64,
    #[serde(rename = "userId")]
	pub user_id: u64,
	pub store: String,
    #[serde(rename = "itemsCount")]
	pub items_count: u64,
    #[serde(rename = "createdAt")]
	pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
	pub updated_at: DateTime<Utc>
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct OrderWithItemResponse {
	pub id: u64,
    #[serde(rename = "userId")]
	pub user_id: u64,
	pub store: String,
    #[serde(rename = "itemsCount")] 
	pub items_count: u64,
	#[serde(rename = "createdAt")]
	pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
	pub updated_at: DateTime<Utc>,
	pub items: Vec<ItemResponse> 
}

#[derive(Serialize)]
pub struct ItemRequest {
    #[serde(rename = "productId")]
	pub product_id: u64,
	pub quantity: u64
}

#[derive(Serialize)]
pub struct ItemUpdateRequest {
	pub quantity: u64
}

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct ItemResponse {
	pub id: u64,
    #[serde(rename = "productId")]
	pub product_id: u64,
	pub title: String,
	pub quantity: u64,
}

impl ApiClient {
    pub async fn create_order(&self, token: &str, store_id: u64) -> anyhow::Result<OrderWithItemResponse> {
        let url = format!("{}/api/private/orders", self.base_url);
        let order_data = CreateOrderRequest { store_id };

        let response = self.client.post(&url).json(&order_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let order: OrderWithItemResponse = response.json().await?;
        Ok(order)
    }

    pub async fn get_order(&self, token: &str, order_id: u64) -> anyhow::Result<OrderWithItemResponse> {
        let url = format!("{}/api/private/orders/{}", self.base_url, order_id);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let order: OrderWithItemResponse = response.json().await?;
        Ok(order)
    }

    pub async fn delete_order(&self, token: &str, order_id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/orders/{}", self.base_url, order_id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;
        Ok("Order deleted successfully".to_string())
    }

    pub async fn list_orders(&self, token: &str) -> anyhow::Result<Vec<OrderResponse>> {
        let url = format!("{}/api/private/orders", self.base_url);

        let response = self.client.get(&url).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let orders: Vec<OrderResponse> = response.json().await?;
        Ok(orders)
    }

    // Order Items

    pub async fn add_item(&self, token: &str, order_id: u64, product_id: u64, quantity: u64) -> anyhow::Result<ItemResponse> {
        let url = format!("{}/api/private/orders/{}/items", self.base_url, order_id);
        let item_data = ItemRequest { product_id, quantity };

        let response = self.client.post(&url).json(&item_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let item: ItemResponse = response.json().await?;
        Ok(item)
    }

    pub async fn update_item(&self, token: &str, order_id: u64, product_id: u64, quantity: u64) -> anyhow::Result<ItemResponse> {
        let url = format!("{}/api/private/orders/{}/items/{}", self.base_url, order_id, product_id);
        let item_data = ItemUpdateRequest { quantity };

        let response = self.client.patch(&url).json(&item_data).bearer_auth(token).send().await?;
        let response = Self::check_status(response).await?;
        let item: ItemResponse = response.json().await?;
        Ok(item)
    }

    pub async fn delete_item(&self, token: &str, order_id: u64, product_id: u64) -> anyhow::Result<String> {
        let url = format!("{}/api/private/orders/{}/items/{}", self.base_url, order_id, product_id);

        let response = self.client.delete(&url).bearer_auth(token).send().await?;
        Self::check_status(response).await?;
        Ok("Item deleted successfully".to_string())
    }
}

#[tauri::command]
pub async fn create_order(state: tauri::State<'_, AppState>, token: String, store_id: u64) -> Result<OrderWithItemResponse, String> {
    state.client.create_order(&token, store_id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn get_order(state: tauri::State<'_, AppState>, token: String, order_id: u64) -> Result<OrderWithItemResponse, String> {
    state.client.get_order(&token, order_id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_order(state: tauri::State<'_, AppState>, token: String, order_id: u64) -> Result<String, String> {
    state.client.delete_order(&token, order_id).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn list_orders(state: tauri::State<'_, AppState>, token: String) -> Result<Vec<OrderResponse>, String> {
    state.client.list_orders(&token).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn add_item(state: tauri::State<'_, AppState>, token: String, order_id: u64, product_id: u64, quantity: u64) -> Result<ItemResponse, String> {
    state.client.add_item(&token, order_id, product_id, quantity).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn update_item(state: tauri::State<'_, AppState>, token: String, order_id: u64, product_id: u64, quantity: u64) -> Result<ItemResponse, String> {
    state.client.update_item(&token, order_id, product_id, quantity).await.map_err(|e|e.to_string())
}

#[tauri::command]
pub async fn delete_item(state: tauri::State<'_, AppState>, token: String, order_id: u64, product_id: u64) -> Result<String, String> {
    state.client.delete_item(&token, order_id, product_id).await.map_err(|e|e.to_string())
}