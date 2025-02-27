#[cfg(not(target_arch = "wasm32"))]
mod native {
    use serde::{de::DeserializeOwned, Serialize};
    pub fn save<T:Serialize>(name:&str, t:&T) {
        let Ok(json) = serde_json::to_string_pretty(t) else { return };
        let _ = std::fs::write(format!("{}.json", name), json);
    }
    
    pub fn load<T:DeserializeOwned>(name:&str) -> Option<T> {
        let Ok(file) = std::fs::read_to_string(format!("{}.json", name)) else { return None };
        let Ok(t) = serde_json::from_str::<T>(&file) else { return None; };
        Some(t)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod web {
    use serde::{de::DeserializeOwned, Serialize};
    pub fn save<T:Serialize>(name:&str, t:&T) {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        let Ok(json) = serde_json::to_string_pretty(t) else { return };
        let _ = storage.set_item(name, &json);
    }
    
    pub fn load<T:DeserializeOwned>(name:&str) -> Option<T> {
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();
        let Some(file) = storage.get_item(name).unwrap() else { return None };
        let Ok(t) = serde_json::from_str::<T>(&file) else { return None; };
        Some(t)
    }
}

#[cfg(target_arch = "wasm32")]
pub use web::*;