use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    query::{CursorQuery, OffsetQuery, WriteBackQuery},
    Error, Feedback, Item, Method, Result, RowAffected, Score, StatusCode, User, Users,
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

    pub async fn insert_user(&self, user: &User) -> Result<RowAffected> {
        return self
            .request(Method::POST, format!("{}api/user", self.entry_point), user)
            .await;
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        return self
            .request::<(), User>(
                Method::GET,
                format!("{}api/user/{}", self.entry_point, user_id),
                &(),
            )
            .await;
    }

    pub async fn delete_user(&self, user_id: &str) -> Result<RowAffected> {
        return self
            .request::<(), RowAffected>(
                Method::DELETE,
                format!("{}api/user/{}", self.entry_point, user_id),
                &(),
            )
            .await;
    }

    pub async fn update_user(&self, user: &User) -> Result<RowAffected> {
        return self
            .request(
                Method::PATCH,
                format!("{}api/user/{}", self.entry_point, user.user_id),
                user,
            )
            .await;
    }

    pub async fn list_users(&self, query: &CursorQuery) -> Result<Vec<User>> {
        return self
            .request::<(), Users>(
                Method::GET,
                format!(
                    "{}api/users?{}",
                    self.entry_point,
                    serde_url_params::to_string(query)?
                ),
                &(),
            )
            .await
            .map(|users| users.users);
    }

    pub async fn insert_users(&self, users: &Vec<User>) -> Result<RowAffected> {
        return self
            .request(
                Method::POST,
                format!("{}api/users", self.entry_point),
                users,
            )
            .await;
    }

    pub async fn insert_item(&self, item: &Item) -> Result<RowAffected> {
        return self
            .request(Method::POST, format!("{}api/item", self.entry_point), item)
            .await;
    }

    pub async fn get_item(&self, item_id: &str) -> Result<Item> {
        return self
            .request::<(), Item>(
                Method::GET,
                format!("{}api/item/{}", self.entry_point, item_id),
                &(),
            )
            .await;
    }

    pub async fn delete_item(&self, item_id: &str) -> Result<RowAffected> {
        return self
            .request::<(), RowAffected>(
                Method::DELETE,
                format!("{}api/item/{}", self.entry_point, item_id),
                &(),
            )
            .await;
    }

    pub async fn insert_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
        return self
            .request(
                Method::POST,
                format!("{}api/feedback", self.entry_point),
                feedback,
            )
            .await;
    }

    pub async fn list_feedback(&self, user_id: &str, feedback_type: &str) -> Result<Vec<Feedback>> {
        return self
            .request::<(), Vec<Feedback>>(
                Method::GET,
                format!(
                    "{}api/user/{}/feedback/{}",
                    self.entry_point, user_id, feedback_type
                ),
                &(),
            )
            .await;
    }

    pub async fn get_item_neighbors(
        &self,
        item_id: &str,
        query: &OffsetQuery,
    ) -> Result<Vec<Score>> {
        return self
            .request::<(), Vec<Score>>(
                Method::GET,
                format!(
                    "{}api/item/{}/neighbors?{}",
                    self.entry_point,
                    item_id,
                    serde_url_params::to_string(query)?
                ),
                &(),
            )
            .await;
    }

    pub async fn get_recommend(
        &self,
        user_id: &str,
        query: &WriteBackQuery,
    ) -> Result<Vec<String>> {
        return self
            .request::<(), Vec<String>>(
                Method::GET,
                format!(
                    "{}api/recommend/{}?{}",
                    self.entry_point,
                    user_id,
                    serde_url_params::to_string(query)?
                ),
                &(),
            )
            .await;
    }

    async fn request<BodyType: Serialize, RetType: for<'a> Deserialize<'a>>(
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
            .send()
            .await?;
        return if response.status() == StatusCode::OK {
            let r: RetType = serde_json::from_str(response.text().await?.as_str())?;
            Ok(r)
        } else {
            Err(Box::new(Error {
                status_code: response.status(),
                message: response.text().await?,
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

    #[tokio::test]
    async fn test_users() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let mut user = User::new("1", vec!["a", "b", "c"]);
        // Insert a user.
        let rows_affected = client.insert_user(&user).await?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this user.
        let return_user = client.get_user("1").await?;
        assert_eq!(return_user, user);
        // Update this user.
        user.labels = vec!["e".into(), "f".into(), "g".into()];
        let rows_affected = client.update_user(&user).await?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this user.
        let return_user = client.get_user("1").await?;
        assert_eq!(return_user, user);
        // Delete this user.
        let rows_affected = client.delete_user("1").await?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_user("1").await;
        assert!(response.is_err());
        // Insert a users.
        let users = vec![user, User::new("10", vec!["a", "b", "c"])];
        let rows_affected = client.insert_users(&users).await?;
        assert_eq!(rows_affected.row_affected, 2);
        // Get this users.
        let return_users = client.list_users(&CursorQuery::new()).await?;
        assert!(!return_users.is_empty());
        // Delete this users.
        let rows_affected = client.delete_user("1").await?;
        assert_eq!(rows_affected.row_affected, 1);
        let rows_affected = client.delete_user("10").await?;
        assert_eq!(rows_affected.row_affected, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_items() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let item = Item::new(
            "1",
            vec!["a", "b", "c"],
            vec!["d", "e"],
            "2022-11-20T13:55:27Z",
        )
        .comment("comment");
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
        let scores = client.get_item_neighbors("10", &OffsetQuery::new()).await?;
        assert_eq!(
            scores,
            vec![
                Score {
                    id: "30".into(),
                    score: 3.0
                },
                Score {
                    id: "20".into(),
                    score: 2.0
                },
                Score {
                    id: "10".into(),
                    score: 1.0
                },
            ]
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_recommend() -> Result<()> {
        let redis = redis::Client::open("redis://127.0.0.1/")?;
        let mut connection = redis.get_connection()?;
        connection.zadd_multiple("offline_recommend/10", &[(1, 10), (2, 20), (3, 30)])?;
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let items = client.get_recommend("10", &WriteBackQuery::new()).await?;
        assert_eq!(
            items,
            vec!["30".to_string(), "20".to_string(), "10".to_string()]
        );
        Ok(())
    }
}