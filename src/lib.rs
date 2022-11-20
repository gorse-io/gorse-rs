use std::{error, fmt};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use reqwest::{Method, StatusCode};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub struct Gorse {
    entry_point: String,
    api_key: String,
    client: Client,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RowAffected {
    #[serde(rename = "RowAffected")]
    pub row_affected: i32,
}

#[derive(Debug)]
struct Error {
    pub status_code: StatusCode,
    pub message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status_code.to_string(), self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl Gorse {
    pub fn new(entry_point: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            entry_point: entry_point.into(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub fn insert_user(&self, user: &User) -> Result<RowAffected> {
        return self.request(Method::POST, format!("{}api/user", self.entry_point), Some(user));
    }

    pub fn get_user(&self, user_id: &str) -> Result<User> {
        return self.request::<(), User>(Method::GET, format!("{}api/user/{}", self.entry_point, user_id), None);
    }

    pub fn delete_user(&self, user_id: &str) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(Method::DELETE, format!("{}api/user/{}", self.entry_point, user_id), None);
    }

    fn request<BodyType: Serialize, RetType: for <'a> Deserialize<'a>>(&self, method: Method, url: String, body: Option<&BodyType>) -> Result<RetType> {
        let mut request = self.client.request(method, url)
            .header("X-API-Key", self.api_key.as_str())
            .header("Content-Type", "application/json");
        if let Some(b) = body {
            request = request.json(b);
        }
        let response = request.send()?;
        return if response.status() == StatusCode::OK {
            let r: RetType = serde_json::from_str(response.text()?.as_str())?;
            Ok(r)
        } else {
            Err(Box::new(Error { status_code: response.status(), message: response.text()? }))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::{*};

    const ENTRY_POINT: &str = "http://127.0.0.1:8088/";
    const API_KEY: &str = "zhenghaoz";

    #[test]
    fn test_users() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        // Insert a user.
        let rows_affected = client.insert_user(&User { user_id: "100".into(), labels: vec!["a".into(), "b".into(), "c".into()] })?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this user.
        let user = client.get_user("100")?;
        assert_eq!(user, User { user_id: "100".into(), labels: vec!["a".into(), "b".into(), "c".into()] });
        // Delete this user.
        let rows_affected = client.delete_user("100")?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_user("100");
        assert!(response.is_err());
        Ok(())
    }
}
