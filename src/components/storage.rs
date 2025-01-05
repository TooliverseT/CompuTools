use web_sys::{window, Storage};

pub struct StorageManager {
    local_storage: Storage,
}

impl StorageManager {
    pub fn new() -> Self {
        let window = window().unwrap();
        let local_storage = window.local_storage().unwrap().unwrap();
        Self { local_storage }
    }

    pub fn get_theme_from_storage(&self) -> String {
        self.local_storage
            .get_item("data-theme")
            .unwrap_or(Some("light".to_string()))
            .unwrap_or_else(|| "light".to_string())
    }

    pub fn save_theme_to_storage(&self, theme: &str) {
        self.local_storage.set_item("data-theme", theme).unwrap();
    }

    pub fn update_html_theme(&self, theme: &str) {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();
        html.set_attribute("data-theme", theme).unwrap();
    }
}
