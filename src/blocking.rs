use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{
    query::{OffsetQuery, WriteBackQuery},
    Error, Feedback, Item, Method, Result, RowAffected, Score, StatusCode, User,
};

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
        return self.request::<(), User>(
            Method::GET,
            format!("{}api/user/{}", self.entry_point, user_id),
            &(),
        );
    }

    pub fn delete_user(&self, user_id: &str) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(
            Method::DELETE,
            format!("{}api/user/{}", self.entry_point, user_id),
            &(),
        );
    }

    pub fn insert_item(&self, item: &Item) -> Result<RowAffected> {
        return self.request(Method::POST, format!("{}api/item", self.entry_point), item);
    }

    pub fn get_item(&self, item_id: &str) -> Result<Item> {
        return self.request::<(), Item>(
            Method::GET,
            format!("{}api/item/{}", self.entry_point, item_id),
            &(),
        );
    }

    pub fn delete_item(&self, item_id: &str) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(
            Method::DELETE,
            format!("{}api/item/{}", self.entry_point, item_id),
            &(),
        );
    }

    pub fn insert_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
        return self.request(
            Method::POST,
            format!("{}api/feedback", self.entry_point),
            feedback,
        );
    }

    pub fn list_feedback(&self, user_id: &str, feedback_type: &str) -> Result<Vec<Feedback>> {
        return self.request::<(), Vec<Feedback>>(
            Method::GET,
            format!(
                "{}api/user/{}/feedback/{}",
                self.entry_point, user_id, feedback_type
            ),
            &(),
        );
    }

    pub fn get_item_neighbors(&self, item_id: &str, query: &OffsetQuery) -> Result<Vec<Score>> {
        return self.request::<(), Vec<Score>>(
            Method::GET,
            format!(
                "{}api/item/{}/neighbors?{}",
                self.entry_point,
                item_id,
                serde_url_params::to_string(query)?
            ),
            &(),
        );
    }

    pub fn get_recommend(&self, user_id: &str, query: &WriteBackQuery) -> Result<Vec<String>> {
        return self.request::<(), Vec<String>>(
            Method::GET,
            format!(
                "{}api/recommend/{}?{}",
                self.entry_point,
                user_id,
                serde_url_params::to_string(query)?
            ),
            &(),
        );
    }

    fn request<BodyType: Serialize, RetType: for<'a> Deserialize<'a>>(
        &self,
        method: Method,
        url: String,
        body: &BodyType,
    ) -> Result<RetType> {
        let response = self
            .client
            .request(method, url)
            .header("X-API-Key", self.api_key.as_str())
            .header("Content-Type", "application/json")
            .json(body)
            .send()?;
        return if response.status() == StatusCode::OK {
            let r: RetType = serde_json::from_str(response.text()?.as_str())?;
            Ok(r)
        } else {
            Err(Box::new(Error {
                status_code: response.status(),
                message: response.text()?,
            }))
        };
    }
}

#[cfg(test)]
mod tests {
    use redis::Commands;

    use super::*;

    const ENTRY_POINT: &str = "http://127.0.0.1:8088/";
    const API_KEY: &str = "zhenghaoz";

    #[test]
    fn test_users() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let user = User::new("100", vec!["a", "b", "c"]);
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
        let item = Item::new(
            "100",
            vec!["a", "b", "c"],
            vec!["d", "e"],
            "2022-11-20T13:55:27Z",
        )
        .comment("comment");
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
        let scores = client.get_item_neighbors("1000", &OffsetQuery::new())?;
        assert_eq!(
            scores,
            vec![
                Score {
                    id: "3000".into(),
                    score: 3.0
                },
                Score {
                    id: "2000".into(),
                    score: 2.0
                },
                Score {
                    id: "1000".into(),
                    score: 1.0
                },
            ]
        );
        Ok(())
    }

    #[test]
    fn test_recommend() -> Result<()> {
        let redis = redis::Client::open("redis://127.0.0.1/")?;
        let mut connection = redis.get_connection()?;
        connection.zadd_multiple("offline_recommend/1000", &[(1, 1000), (2, 2000), (3, 3000)])?;
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let items = client.get_recommend("1000", &WriteBackQuery::new())?;
        assert_eq!(
            items,
            vec!["3000".to_string(), "2000".to_string(), "1000".to_string()]
        );
        Ok(())
    }
}
