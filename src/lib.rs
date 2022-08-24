use std::sync::Arc;

use reqwest::Client;

#[derive(Debug, Clone)]
pub struct Gorse {
    pub(crate) entry_point: Arc<String>,
    pub(crate) api_key: Arc<String>,
    client: Client,
}

impl Gorse {
    pub fn new(entry_point: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            entry_point: Arc::new(entry_point.into()),
            api_key: Arc::new(api_key.into()),
            client: Client::new(),
        }
    }

    pub fn entry_point(&self) -> String {
        self.entry_point.to_string()
    }

    pub fn api_key(&self) -> String {
        self.api_key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    #[test]
    fn client_defination() {
        let entry_point = "http://127.0.0.1:8088/";
        let api_key = "zhenghaoz";
        let client = Gorse::new(entry_point, api_key);
        assert_eq!(client.entry_point(), entry_point);
        assert_eq!(client.api_key(), api_key);
    }
}
