use std::{error, fmt};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use reqwest::{Method, StatusCode};
use reqwest::Client;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,
    #[serde(rename = "Categories")]
    pub categories: Vec<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "Comment")]
    pub comment: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    #[serde(rename = "FeedbackType")]
    pub feedback_type: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
}

impl Feedback {
    pub fn new(feedback_type: impl Into<String>, user_id: impl Into<String>, item_id: impl Into<String>, timestamp: impl Into<String>) -> Self {
        return Feedback {
            feedback_type: feedback_type.into(),
            user_id: user_id.into(),
            item_id: item_id.into(),
            timestamp: timestamp.into(),
        };
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RowAffected {
    #[serde(rename = "RowAffected")]
    pub row_affected: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Score {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Score")]
    pub score: f64,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct Gorse {
    entry_point: String,
    api_key: String,
    client: Client,
}

impl Gorse {
    pub fn new(entry_point: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            entry_point: entry_point.into(),
            api_key: api_key.into(),
            client: Client::new(),
        }
    }

    pub async fn insert_user(&self, user: &User) -> Result<RowAffected> {
        return self.request(Method::POST, format!("{}api/user", self.entry_point), user).await;
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        return self.request::<(), User>(Method::GET, format!("{}api/user/{}", self.entry_point, user_id), &()).await;
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(Method::DELETE, format!("{}api/user/{}", self.entry_point, user_id), &()).await;
    }

    pub async fn insert_item(&self, item: &Item) -> Result<RowAffected> {
        return self.request(Method::POST, format!("{}api/item", self.entry_point), item).await;
    }

    pub async fn get_item(&self, item_id: &str) -> Result<Item> {
        return self.request::<(), Item>(Method::GET, format!("{}api/item/{}", self.entry_point, item_id), &()).await;
    }

    pub async fn delete_item(&self, item_id: &str) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(Method::DELETE, format!("{}api/item/{}", self.entry_point, item_id), &()).await;
    }

    pub async fn insert_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
        return self.request(Method::POST, format!("{}api/feedback", self.entry_point), feedback).await;
    }

    pub async fn list_feedback(&self, user_id: &str, feedback_type: &str) -> Result<Vec<Feedback>> {
        return self.request::<(), Vec<Feedback>>(Method::GET, format!("{}api/user/{}/feedback/{}", self.entry_point, user_id, feedback_type), &()).await;
    }

    pub async fn get_item_neighbors(&self, item_id: &str) -> Result<Vec<Score>> {
        return self.request::<(), Vec<Score>>(Method::GET, format!("{}api/item/{}/neighbors", self.entry_point, item_id), &()).await;
    }

    pub async fn get_recommend(&self, user_id: &str) -> Result<Vec<String>> {
        return self.request::<(), Vec<String>>(Method::GET, format!("{}api/recommend/{}", self.entry_point, user_id), &()).await;
    }

    async fn request<BodyType: Serialize, RetType: for<'a> Deserialize<'a>>(&self, method: Method, url: String, body: &BodyType) -> Result<RetType> {
        let response = self.client.request(method, url)
            .header("X-API-Key", self.api_key.as_str())
            .header("Content-Type", "application/json")
            .json(body)
            .send().await?;
        return if response.status() == StatusCode::OK {
            let r: RetType = serde_json::from_str(response.text().await?.as_str())?;
            Ok(r)
        } else {
            Err(Box::new(Error { status_code: response.status(), message: response.text().await? }))
        };
    }
}

#[cfg(test)]
mod tests {
    use redis::Commands;

    use super::{*};

    const ENTRY_POINT: &str = "http://127.0.0.1:8088/";
    const API_KEY: &str = "zhenghaoz";

    #[tokio::test]
    async fn test_users() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let user = User { user_id: "1".into(), labels: vec!["a".into(), "b".into(), "c".into()] };
        // Insert a user.
        let rows_affected = client.insert_user(&user).await?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this user.
        let return_user = client.get_user("1").await?;
        assert_eq!(return_user, user);
        // Delete this user.
        let rows_affected = client.delete_user("1").await?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_user("1").await;
        assert!(response.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_items() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let item = Item {
            item_id: "1".into(),
            is_hidden: true,
            labels: vec!["a".into(), "b".into(), "c".into()],
            categories: vec!["d".into(), "e".into()],
            timestamp: "2022-11-20T13:55:27Z".into(),
            comment: "comment".into(),
        };
        // Insert an item.
        let rows_affected = client.insert_item(&item).await?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this item.
        let return_item = client.get_item("1").await?;
        assert_eq!(return_item, item);
        // Delete this item.
        let rows_affected = client.delete_item("1").await?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_item("1").await;
        assert!(response.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_feedback() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let feedback = vec![
            Feedback::new("read", "10", "3", "2022-11-20T13:55:27Z"),
            Feedback::new("read", "10", "4", "2022-11-20T13:55:27Z"),
        ];
        let rows_affected = client.insert_feedback(&feedback).await?;
        assert_eq!(rows_affected.row_affected, 2);
        // Insert feedback.
        let return_feedback = client.list_feedback("10", "read").await?;
        assert_eq!(return_feedback, feedback);
        Ok(())
    }

    #[tokio::test]
    async fn test_neighbors() -> Result<()> {
        let redis = redis::Client::open("redis://127.0.0.1/")?;
        let mut connection = redis.get_connection()?;
        connection.zadd_multiple("item_neighbors/10", &[(1, 10), (2, 20), (3, 30)])?;
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let scores = client.get_item_neighbors("10").await?;
        assert_eq!(scores, vec![
            Score { id: "30".into(), score: 3.0 },
            Score { id: "20".into(), score: 2.0 },
            Score { id: "10".into(), score: 1.0 },
        ]);
        Ok(())
    }

    #[tokio::test]
    async fn test_recommend() -> Result<()> {
        let redis = redis::Client::open("redis://127.0.0.1/")?;
        let mut connection = redis.get_connection()?;
        connection.zadd_multiple("offline_recommend/10", &[(1, 10), (2, 20), (3, 30)])?;
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let items = client.get_recommend("10").await?;
        assert_eq!(items, vec!["30".to_string(), "20".to_string(), "10".to_string()]);
        Ok(())
    }
}

pub mod blocking {
    use reqwest::blocking::Client;
    use serde::{Deserialize, Serialize};

    use crate::{Error, Feedback, Item, Method, Result, RowAffected, Score, StatusCode, User};

    #[derive(Debug, Clone)]
    pub struct Gorse {
        entry_point: String,
        api_key: String,
        client: Client,
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
            return self.request(Method::POST, format!("{}api/user", self.entry_point), user);
        }

        pub fn get_user(&self, user_id: &str) -> Result<User> {
            return self.request::<(), User>(Method::GET, format!("{}api/user/{}", self.entry_point, user_id), &());
        }

        pub fn delete_user(&self, user_id: &str) -> Result<RowAffected> {
            return self.request::<(), RowAffected>(Method::DELETE, format!("{}api/user/{}", self.entry_point, user_id), &());
        }

        pub fn insert_item(&self, item: &Item) -> Result<RowAffected> {
            return self.request(Method::POST, format!("{}api/item", self.entry_point), item);
        }

        pub fn get_item(&self, item_id: &str) -> Result<Item> {
            return self.request::<(), Item>(Method::GET, format!("{}api/item/{}", self.entry_point, item_id), &());
        }

        pub fn delete_item(&self, item_id: &str) -> Result<RowAffected> {
            return self.request::<(), RowAffected>(Method::DELETE, format!("{}api/item/{}", self.entry_point, item_id), &());
        }

        pub fn insert_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
            return self.request(Method::POST, format!("{}api/feedback", self.entry_point), feedback);
        }

        pub fn list_feedback(&self, user_id: &str, feedback_type: &str) -> Result<Vec<Feedback>> {
            return self.request::<(), Vec<Feedback>>(Method::GET, format!("{}api/user/{}/feedback/{}", self.entry_point, user_id, feedback_type), &());
        }

        pub fn get_item_neighbors(&self, item_id: &str) -> Result<Vec<Score>> {
            return self.request::<(), Vec<Score>>(Method::GET, format!("{}api/item/{}/neighbors", self.entry_point, item_id), &());
        }

        pub fn get_recommend(&self, user_id: &str) -> Result<Vec<String>> {
            return self.request::<(), Vec<String>>(Method::GET, format!("{}api/recommend/{}", self.entry_point, user_id), &());
        }

        fn request<BodyType: Serialize, RetType: for<'a> Deserialize<'a>>(&self, method: Method, url: String, body: &BodyType) -> Result<RetType> {
            let response = self.client.request(method, url)
                .header("X-API-Key", self.api_key.as_str())
                .header("Content-Type", "application/json")
                .json(body)
                .send()?;
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
        use redis::Commands;

        use super::{*};

        const ENTRY_POINT: &str = "http://127.0.0.1:8088/";
        const API_KEY: &str = "zhenghaoz";

        #[test]
        fn test_users() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let user = User { user_id: "100".into(), labels: vec!["a".into(), "b".into(), "c".into()] };
            // Insert a user.
            let rows_affected = client.insert_user(&user)?;
            assert_eq!(rows_affected.row_affected, 1);
            // Get this user.
            let return_user = client.get_user("100")?;
            assert_eq!(return_user, user);
            // Delete this user.
            let rows_affected = client.delete_user("100")?;
            assert_eq!(rows_affected.row_affected, 1);
            let response = client.get_user("100");
            assert!(response.is_err());
            Ok(())
        }

        #[test]
        fn test_items() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let item = Item {
                item_id: "100".into(),
                is_hidden: true,
                labels: vec!["a".into(), "b".into(), "c".into()],
                categories: vec!["d".into(), "e".into()],
                timestamp: "2022-11-20T13:55:27Z".into(),
                comment: "comment".into(),
            };
            // Insert an item.
            let rows_affected = client.insert_item(&item)?;
            assert_eq!(rows_affected.row_affected, 1);
            // Get this item.
            let return_item = client.get_item("100")?;
            assert_eq!(return_item, item);
            // Delete this item.
            let rows_affected = client.delete_item("100")?;
            assert_eq!(rows_affected.row_affected, 1);
            let response = client.get_item("100");
            assert!(response.is_err());
            Ok(())
        }

        #[test]
        fn test_feedback() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let feedback = vec![
                Feedback::new("read", "1000", "300", "2022-11-20T13:55:27Z"),
                Feedback::new("read", "1000", "400", "2022-11-20T13:55:27Z"),
            ];
            let rows_affected = client.insert_feedback(&feedback)?;
            assert_eq!(rows_affected.row_affected, 2);
            // Insert feedback.
            let return_feedback = client.list_feedback("1000", "read")?;
            assert_eq!(return_feedback, feedback);
            Ok(())
        }

        #[test]
        fn test_neighbors() -> Result<()> {
            let redis = redis::Client::open("redis://127.0.0.1/")?;
            let mut connection = redis.get_connection()?;
            connection.zadd_multiple("item_neighbors/1000", &[(1, 1000), (2, 2000), (3, 3000)])?;
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let scores = client.get_item_neighbors("1000")?;
            assert_eq!(scores, vec![
                Score { id: "3000".into(), score: 3.0 },
                Score { id: "2000".into(), score: 2.0 },
                Score { id: "1000".into(), score: 1.0 },
            ]);
            Ok(())
        }

        #[test]
        fn test_recommend() -> Result<()> {
            let redis = redis::Client::open("redis://127.0.0.1/")?;
            let mut connection = redis.get_connection()?;
            connection.zadd_multiple("offline_recommend/1000", &[(1, 1000), (2, 2000), (3, 3000)])?;
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let items = client.get_recommend("1000")?;
            assert_eq!(items, vec!["3000".to_string(), "2000".to_string(), "1000".to_string()]);
            Ok(())
        }
    }
}
