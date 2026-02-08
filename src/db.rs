use rexie::{Error, ObjectStore, Rexie, TransactionMode};
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub created_at: u64,
}

#[cfg(feature = "ssr")]
pub async fn init_db() -> Result<Rexie, String> {
    Err("IndexedDB not available on server".to_string())
}

#[cfg(feature = "ssr")]
pub async fn save_user(_user: User) -> Result<(), String> {
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_user() -> Result<Option<User>, String> {
    Ok(None)
}

#[cfg(not(feature = "ssr"))]
pub async fn init_db() -> Result<Rexie, String> {
    let rexie = Rexie::builder("chat_stream_db")
        .version(1)
        .add_object_store(ObjectStore::new("users").auto_increment(true))
        .build()
        .await
        .map_err(|e| e.to_string())?;

    Ok(rexie)
}

#[cfg(not(feature = "ssr"))]
pub async fn save_user(user: User) -> Result<(), String> {
    let rexie = init_db().await?;
    let transaction = rexie
        .transaction(&["users"], TransactionMode::ReadWrite)
        .map_err(|e| e.to_string())?;
    let users_store = transaction.store("users").map_err(|e| e.to_string())?;

    let user_js_value =
        serde_wasm_bindgen::to_value(&user).map_err(|e| format!("Serialization error: {}", e))?;

    users_store.clear().await.map_err(|e| e.to_string())?;

    users_store
        .add(&user_js_value, None)
        .await
        .map_err(|e| e.to_string())?;
    transaction.done().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub async fn get_user() -> Result<Option<User>, String> {
    let rexie = init_db().await?;
    let transaction = rexie
        .transaction(&["users"], TransactionMode::ReadOnly)
        .map_err(|e| e.to_string())?;
    let users_store = transaction.store("users").map_err(|e| e.to_string())?;

    let users = users_store
        .get_all(None, None)
        .await
        .map_err(|e| e.to_string())?;

    if users.is_empty() {
        return Ok(None);
    }

    // user_pair is actually just the JsValue, not a pair since get_all returns values
    let user_js = users.get(0).ok_or("No user found".to_string())?;
    let user: User = serde_wasm_bindgen::from_value(user_js.clone())
        .map_err(|e| format!("Deserialization error: {}", e))?;

    Ok(Some(user))
}
