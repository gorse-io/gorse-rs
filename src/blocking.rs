use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::{
    query::{CursorQuery, OffsetQuery, WriteBackQuery},
    Error, Feedback, Feedbacks, Health, Item, Items, Method, Result, RowAffected, Score,
    StatusCode, User, Users,
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

    pub fn update_user(&self, user: &User) -> Result<RowAffected> {
        return self.request(
            Method::PATCH,
            format!("{}api/user/{}", self.entry_point, user.user_id),
            user,
        );
    }

    pub fn list_users(&self, query: &CursorQuery) -> Result<Vec<User>> {
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
            .map(|users| users.users);
    }

    pub fn insert_users(&self, users: &Vec<User>) -> Result<RowAffected> {
        return self.request(
            Method::POST,
            format!("{}api/users", self.entry_point),
            users,
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

    pub fn update_item(&self, item: &Item) -> Result<RowAffected> {
        return self.request(
            Method::PATCH,
            format!("{}api/item/{}", self.entry_point, item.item_id),
            item,
        );
    }

    pub fn add_item_to_category(&self, item_id: &str, category: &str) -> Result<RowAffected> {
        return self.request(
            Method::PUT,
            format!(
                "{}api/item/{}/category/{}",
                self.entry_point, item_id, category
            ),
            &(),
        );
    }

    pub fn delete_item_to_category(&self, item_id: &str, category: &str) -> Result<RowAffected> {
        return self.request(
            Method::DELETE,
            format!(
                "{}api/item/{}/category/{}",
                self.entry_point, item_id, category
            ),
            &(),
        );
    }

    pub fn list_items(&self, query: &CursorQuery) -> Result<Vec<Item>> {
        return self
            .request::<(), Items>(
                Method::GET,
                format!(
                    "{}api/items?{}",
                    self.entry_point,
                    serde_url_params::to_string(query)?
                ),
                &(),
            )
            .map(|items| items.items);
    }

    pub fn insert_items(&self, items: &Vec<Item>) -> Result<RowAffected> {
        return self.request(
            Method::POST,
            format!("{}api/items", self.entry_point),
            items,
        );
    }

    pub fn list_feedback(&self, query: &CursorQuery) -> Result<Vec<Feedback>> {
        return self
            .request::<(), Feedbacks>(
                Method::GET,
                format!(
                    "{}api/feedback?{}",
                    self.entry_point,
                    serde_url_params::to_string(query)?
                ),
                &(),
            )
            .map(|feedbacks| feedbacks.feedbacks);
    }

    pub fn overwrite_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
        return self.request(
            Method::PUT,
            format!("{}api/feedback", self.entry_point),
            feedback,
        );
    }

    pub fn insert_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
        return self.request(
            Method::POST,
            format!("{}api/feedback", self.entry_point),
            feedback,
        );
    }

    pub fn list_feedback_by_type(
        &self,
        feedback_type: &str,
        query: &CursorQuery,
    ) -> Result<Vec<Feedback>> {
        return self
            .request::<(), Feedbacks>(
                Method::GET,
                format!(
                    "{}api/feedback/{}?{}",
                    self.entry_point,
                    feedback_type,
                    serde_url_params::to_string(query).unwrap()
                ),
                &(),
            )
            .map(|feedbacks| feedbacks.feedbacks);
    }

    pub fn get_feedback(
        &self,
        feedback_type: &str,
        user_id: &str,
        item_id: &str,
    ) -> Result<Feedback> {
        return self.request::<(), Feedback>(
            Method::GET,
            format!(
                "{}api/feedback/{}/{}/{}",
                self.entry_point, feedback_type, user_id, item_id,
            ),
            &(),
        );
    }

    pub fn delete_feedback(
        &self,
        feedback_type: &str,
        user_id: &str,
        item_id: &str,
    ) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(
            Method::DELETE,
            format!(
                "{}api/feedback/{}/{}/{}",
                self.entry_point, feedback_type, user_id, item_id,
            ),
            &(),
        );
    }

    pub fn list_feedback_from_user_by_item(
        &self,
        user_id: &str,
        item_id: &str,
    ) -> Result<Vec<Feedback>> {
        return self.request::<(), Vec<Feedback>>(
            Method::GET,
            format!("{}api/feedback/{}/{}", self.entry_point, user_id, item_id),
            &(),
        );
    }

    pub fn delete_feedback_from_user_by_item(
        &self,
        user_id: &str,
        item_id: &str,
    ) -> Result<RowAffected> {
        return self.request::<(), RowAffected>(
            Method::DELETE,
            format!("{}api/feedback/{}/{}", self.entry_point, user_id, item_id),
            &(),
        );
    }

    pub fn list_feedback_by_item(&self, item_id: &str) -> Result<Vec<Feedback>> {
        return self.request::<(), Vec<Feedback>>(
            Method::GET,
            format!("{}api/item/{}/feedback", self.entry_point, item_id),
            &(),
        );
    }

    pub fn list_feedback_by_item_and_type(
        &self,
        item_id: &str,
        feedback_type: &str,
    ) -> Result<Vec<Feedback>> {
        return self.request::<(), Vec<Feedback>>(
            Method::GET,
            format!(
                "{}api/item/{}/feedback/{}",
                self.entry_point, item_id, feedback_type
            ),
            &(),
        );
    }

    pub fn list_feedback_from_user(&self, user_id: &str) -> Result<Vec<Feedback>> {
        return self.request::<(), Vec<Feedback>>(
            Method::GET,
            format!("{}api/user/{}/feedback", self.entry_point, user_id),
            &(),
        );
    }

    pub fn list_feedback_from_user_by_type(
        &self,
        user_id: &str,
        feedback_type: &str,
    ) -> Result<Vec<Feedback>> {
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

    pub fn get_item_neighbors_by_category(
        &self,
        item_id: &str,
        category: &str,
        query: &OffsetQuery,
    ) -> Result<Vec<Score>> {
        return self.request::<(), Vec<Score>>(
            Method::GET,
            format!(
                "{}api/item/{}/neighbors/{}?{}",
                self.entry_point,
                item_id,
                category,
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

    pub fn get_user_neighbors(&self, user_id: &str, query: &OffsetQuery) -> Result<Vec<Score>> {
        return self.request::<(), Vec<Score>>(
            Method::GET,
            format!(
                "{}api/user/{}/neighbors?{}",
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

    pub fn is_live(&self) -> Result<Health> {
        return self.request(
            Method::GET,
            format!("{}api/health/live", self.entry_point),
            &(),
        );
    }

    pub fn is_ready(&self) -> Result<Health> {
        return self.request(
            Method::GET,
            format!("{}api/health/ready", self.entry_point),
            &(),
        );
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
        let mut user = User::new("100", vec!["a", "b", "c"]);
        // Insert a user.
        let rows_affected = client.insert_user(&user)?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this user.
        let return_user = client.get_user("100")?;
        assert_eq!(return_user, user);
        // Update this user.
        user.labels = vec!["e".into(), "f".into(), "g".into()];
        let rows_affected = client.update_user(&user)?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this user.
        let return_user = client.get_user("100")?;
        assert_eq!(return_user, user);
        // Delete this user.
        let rows_affected = client.delete_user("100")?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_user("100");
        assert!(response.is_err());
        // Insert a users.
        let users = vec![user, User::new("102", vec!["a", "b", "c"])];
        let rows_affected = client.insert_users(&users)?;
        assert_eq!(rows_affected.row_affected, 2);
        // Get this users.
        let return_users = client.list_users(&CursorQuery::new())?;
        assert!(!return_users.is_empty());
        // Delete this users.
        let rows_affected = client.delete_user("100")?;
        assert_eq!(rows_affected.row_affected, 1);
        let rows_affected = client.delete_user("102")?;
        assert_eq!(rows_affected.row_affected, 1);
        Ok(())
    }

    #[test]
    fn test_items() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let category = "test".to_string();
        let item = Item::new(
            "100",
            vec!["a", "b", "c"],
            vec!["d", "e"],
            "2022-11-20T13:55:27Z",
        )
        .comment("comment");
        let item_with_category = Item::new(
            "100",
            vec!["a", "b", "c"],
            vec!["d", "e", &category],
            "2022-11-20T13:55:27Z",
        )
        .comment("comment");
        // Insert an item.
        let rows_affected = client.insert_item(&item)?;
        assert_eq!(rows_affected.row_affected, 1);
        // Add category to item.
        let rows_affected = client.add_item_to_category(&item.item_id, &category)?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this item.
        let return_item = client.get_item("100")?;
        assert_eq!(return_item, item_with_category);
        // Delete category to item.
        let rows_affected = client.delete_item_to_category(&item.item_id, &category)?;
        assert_eq!(rows_affected.row_affected, 1);
        // Get this item.
        let return_item = client.get_item("100")?;
        assert_eq!(return_item, item);
        // Delete this item.
        let rows_affected = client.delete_item("100")?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_item("100");
        assert!(response.is_err());
        // Insert a items.
        let items = vec![
            item,
            Item::new(
                "102",
                vec!["d", "e"],
                vec!["a", "b", "c"],
                "2023-11-20T13:55:27Z",
            ),
        ];
        let rows_affected = client.insert_items(&items)?;
        assert_eq!(rows_affected.row_affected, 2);
        // Get this items.
        let return_items = client.list_items(&CursorQuery::new())?;
        assert!(!return_items.is_empty());
        // Delete this items.
        let rows_affected = client.delete_item("100")?;
        assert_eq!(rows_affected.row_affected, 1);
        let rows_affected = client.delete_item("102")?;
        assert_eq!(rows_affected.row_affected, 1);
        Ok(())
    }

    #[test]
    fn test_feedback() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let all_feedback = vec![
            Feedback::new("read", "10", "3", "2022-11-20T13:55:27Z"),
            Feedback::new("read", "10", "4", "2022-11-20T13:55:27Z"),
            Feedback::new("read", "1000", "300", "2022-11-20T13:55:27Z"),
            Feedback::new("read", "1000", "400", "2022-11-20T13:55:27Z"),
        ];
        let feedback = vec![
            Feedback::new("read", "1000", "300", "2022-11-20T13:55:27Z"),
            Feedback::new("read", "1000", "400", "2022-11-20T13:55:27Z"),
            Feedback::new("star", "1000", "300", "2022-11-20T13:55:27Z"),
            Feedback::new("read", "1000", "500", "2022-11-20T13:55:27Z"),
            Feedback::new("star", "1000", "500", "2022-11-20T13:55:27Z"),
        ];
        // Insert feedback.
        let rows_affected = client.insert_feedback(&feedback)?;
        assert_eq!(rows_affected.row_affected, 5);
        // Overwrite feedback.
        let rows_affected = client.overwrite_feedback(&feedback)?;
        assert_eq!(rows_affected.row_affected, 5);
        // Delete feedback.
        let rows_affected = client.delete_feedback("star", "1000", "300")?;
        assert_eq!(rows_affected.row_affected, 1);
        let response = client.get_feedback("star", "1000", "300");
        assert!(response.is_err());
        // Delete feedback from user by item.
        let rows_affected = client.delete_feedback_from_user_by_item("1000", "500")?;
        assert_eq!(rows_affected.row_affected, 2);
        // List feedback.
        let return_feedback = client.list_feedback(&CursorQuery::new())?;
        assert_eq!(return_feedback, all_feedback);
        // List feedback by type.
        let return_feedback = client.list_feedback_by_type("read", &CursorQuery::new())?;
        assert_eq!(return_feedback, all_feedback);
        // Get feedback.
        let return_feedback = client.get_feedback("read", "1000", "300")?;
        assert_eq!(return_feedback, feedback[0]);
        // List feedback by item.
        let return_feedback = client.list_feedback_by_item("300")?;
        assert_eq!(return_feedback, feedback[..1]);
        // List feedback by item and type.
        let return_feedback = client.list_feedback_by_item_and_type("300", "read")?;
        assert_eq!(return_feedback, feedback[..1]);
        // List feedback from user.
        let return_feedback = client.list_feedback_from_user("1000")?;
        assert_eq!(return_feedback, feedback[..2]);
        // List feedback from user by item.
        let return_feedback = client.list_feedback_from_user_by_item("1000", "300")?;
        assert_eq!(return_feedback, feedback[..1]);
        // List feedback from user by type.
        let return_feedback = client.list_feedback_from_user_by_type("1000", "read")?;
        assert_eq!(return_feedback, feedback[..2]);
        Ok(())
    }

    #[test]
    fn test_neighbors() -> Result<()> {
        let redis = redis::Client::open("redis://127.0.0.1/")?;
        let mut connection = redis.get_connection()?;
        connection.zadd_multiple("user_neighbors/1000", &[(1, 1000), (2, 2000), (3, 3000)])?;
        connection.zadd_multiple("item_neighbors/1000", &[(1, 1000), (2, 2000), (3, 3000)])?;
        let scores = vec![
            Score {
                id: "3000".into(),
                score: 3.0,
            },
            Score {
                id: "2000".into(),
                score: 2.0,
            },
            Score {
                id: "1000".into(),
                score: 1.0,
            },
        ];
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        // Get item neighbors.
        let returned_scores = client.get_item_neighbors("1000", &OffsetQuery::new())?;
        assert_eq!(returned_scores, scores);
        // Get item neighbors by category.
        let returned_scores =
            client.get_item_neighbors_by_category("1000", "", &OffsetQuery::new())?;
        assert_eq!(returned_scores, scores);
        // Get user neighbors.
        let returned_scores = client.get_user_neighbors("1000", &OffsetQuery::new())?;
        assert_eq!(returned_scores, scores);
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

    #[test]
    fn test_health() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let health = Health {
            cache_store_connected: true,
            cache_store_error: None,
            data_store_connected: true,
            data_store_error: None,
            ready: true,
        };
        let return_health = client.is_live()?;
        assert_eq!(return_health, health);
        let return_health = client.is_ready()?;
        assert_eq!(return_health, health);
        Ok(())
    }
}
