use reqwest::Client;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Labels")]
    pub labels: Value,
    #[serde(rename = "Comment")]
    pub comment: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "IsHidden")]
    pub is_hidden: bool,
    #[serde(rename = "Labels")]
    pub labels: Value,
    #[serde(rename = "Categories")]
    pub categories: Vec<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
    #[serde(rename = "Comment")]
    pub comment: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Feedback {
    #[serde(rename = "FeedbackType")]
    pub feedback_type: String,
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "ItemId")]
    pub item_id: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Timestamp")]
    pub timestamp: String,
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("API error: {status_code}: {message}")]
    Api {
        status_code: StatusCode,
        message: String,
    },
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

#[derive(Default)]
pub struct RecommendOptions {
    pub n: usize,
}

#[derive(Debug, Clone)]
pub struct Gorse {
    entry_point: String,
    api_key: String,
    client: Client,
}

impl Gorse {
    pub fn new(entry_point: impl Into<String>, api_key: impl Into<String>) -> Self {
        let mut entry_point = entry_point.into();
        if !entry_point.ends_with('/') {
            entry_point.push('/');
        }
        Self {
            entry_point,
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

    pub async fn delete_feedback(&self, user_id: &str, item_id: &str) -> Result<RowAffected> {
        return self
            .request::<(), RowAffected>(
                Method::DELETE,
                format!("{}api/feedback/{}/{}", self.entry_point, user_id, item_id),
                &(),
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

    pub async fn get_item_neighbors(&self, item_id: &str) -> Result<Vec<Score>> {
        return self
            .request::<(), Vec<Score>>(
                Method::GET,
                format!("{}api/item/{}/neighbors", self.entry_point, item_id),
                &(),
            )
            .await;
    }

    pub async fn get_recommend(
        &self,
        user_id: &str,
        options: RecommendOptions,
    ) -> Result<Vec<String>> {
        let mut url = format!("{}api/recommend/{}", self.entry_point, user_id);
        if options.n > 0 {
            url = format!("{}?n={}", url, options.n);
        }
        return self.request::<(), Vec<String>>(Method::GET, url, &()).await;
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
        if response.status() == StatusCode::OK {
            let r: RetType = serde_json::from_str(response.text().await?.as_str())?;
            Ok(r)
        } else {
            Err(Error::Api {
                status_code: response.status(),
                message: response.text().await?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use serial_test::serial;

    const ENTRY_POINT: &str = "http://127.0.0.1:8088/";
    const API_KEY: &str = "zhenghaoz";

    #[tokio::test]
    #[serial]
    async fn test_users() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let user = User {
            user_id: "2000".into(),
            labels: json!({
                "gender": "M",
                "occupation": "engineer"
            }),
            comment: "zhenghaoz".into(),
        };
        let r = client.insert_user(&user).await?;
        assert_eq!(r.row_affected, 1);
        let resp = client.get_user("2000").await?;
        assert_eq!(user, resp);

        let r = client.delete_user("2000").await?;
        assert_eq!(r.row_affected, 1);
        match client.get_user("2000").await {
            Ok(_) => panic!("Expected error"),
            Err(Error::Api { status_code, .. }) => {
                assert_eq!(status_code, StatusCode::NOT_FOUND);
            }
            Err(e) => panic!("Expected API error, got {:?}", e),
        }
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_items() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let item = Item {
            item_id: "2000".into(),
            is_hidden: true,
            labels: json!({
                "embedding": [0.1, 0.2, 0.3]
            }),
            categories: vec!["Comedy".into(), "Animation".into()],
            timestamp: "2022-11-20T13:55:27Z".into(),
            comment: "Minions (2015)".into(),
        };
        let r = client.insert_item(&item).await?;
        assert_eq!(r.row_affected, 1);
        let resp = client.get_item("2000").await?;
        assert_eq!(item, resp);

        let r = client.delete_item("2000").await?;
        assert_eq!(r.row_affected, 1);
        match client.get_item("2000").await {
            Ok(_) => panic!("Expected error"),
            Err(Error::Api { status_code, .. }) => {
                assert_eq!(status_code, StatusCode::NOT_FOUND);
            }
            Err(e) => panic!("Expected API error, got {:?}", e),
        }
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_feedback() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        client
            .insert_user(&User {
                user_id: "2000".into(),
                labels: json!({}),
                comment: "".into(),
            })
            .await?;

        let feedbacks = vec![
            Feedback {
                feedback_type: "watch".into(),
                user_id: "2000".into(),
                item_id: "1".into(),
                value: 1.0,
                timestamp: "2022-11-20T13:55:27Z".into(),
            },
            Feedback {
                feedback_type: "watch".into(),
                user_id: "2000".into(),
                item_id: "1060".into(),
                value: 2.0,
                timestamp: "2022-11-20T13:55:27Z".into(),
            },
            Feedback {
                feedback_type: "watch".into(),
                user_id: "2000".into(),
                item_id: "11".into(),
                value: 3.0,
                timestamp: "2022-11-20T13:55:27Z".into(),
            },
        ];
        for fb in &feedbacks {
            client.delete_feedback(&fb.user_id, &fb.item_id).await?;
        }
        let r = client.insert_feedback(&feedbacks).await?;
        assert_eq!(r.row_affected, 3);
        let user_feedback = client.list_feedback("2000", "watch").await?;
        assert_eq!(feedbacks, user_feedback);

        let r = client.delete_feedback("2000", "1").await?;
        assert_eq!(r.row_affected, 1);
        let user_feedback = client.list_feedback("2000", "watch").await?;
        assert_eq!(
            vec![feedbacks[1].clone(), feedbacks[2].clone()],
            user_feedback
        );
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_item_to_item() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        let scores = client.get_item_neighbors("1").await?;
        assert_eq!(scores[0].id, "1060".to_string());
        assert_eq!(scores[1].id, "404".to_string());
        assert_eq!(scores[2].id, "1219".to_string());
        Ok(())
    }

    #[tokio::test]
    #[serial]
    async fn test_recommend() -> Result<()> {
        let client = Gorse::new(ENTRY_POINT, API_KEY);
        client
            .insert_user(&User {
                user_id: "3000".into(),
                labels: json!({}),
                comment: "".into(),
            })
            .await?;
        let items = client
            .get_recommend("3000", RecommendOptions { n: 3 })
            .await?;
        assert_eq!(
            items,
            vec!["315".to_string(), "1432".to_string(), "918".to_string()]
        );
        Ok(())
    }
}

pub mod blocking {
    use reqwest::blocking::Client;
    use serde::{Deserialize, Serialize};

    use crate::{
        Error, Feedback, Item, Method, RecommendOptions, Result, RowAffected, Score, StatusCode,
        User,
    };

    #[derive(Debug, Clone)]
    pub struct Gorse {
        entry_point: String,
        api_key: String,
        client: Client,
    }

    impl Gorse {
        pub fn new(entry_point: impl Into<String>, api_key: impl Into<String>) -> Self {
            let mut entry_point = entry_point.into();
            if !entry_point.ends_with('/') {
                entry_point.push('/');
            }
            Self {
                entry_point,
                api_key: api_key.into(),
                client: Client::new(),
            }
        }

        pub fn insert_user(&self, user: &User) -> Result<RowAffected> {
            self.request(Method::POST, format!("{}api/user", self.entry_point), user)
        }

        pub fn get_user(&self, user_id: &str) -> Result<User> {
            self.request::<(), User>(
                Method::GET,
                format!("{}api/user/{}", self.entry_point, user_id),
                &(),
            )
        }

        pub fn delete_user(&self, user_id: &str) -> Result<RowAffected> {
            self.request::<(), RowAffected>(
                Method::DELETE,
                format!("{}api/user/{}", self.entry_point, user_id),
                &(),
            )
        }

        pub fn insert_item(&self, item: &Item) -> Result<RowAffected> {
            self.request(Method::POST, format!("{}api/item", self.entry_point), item)
        }

        pub fn get_item(&self, item_id: &str) -> Result<Item> {
            self.request::<(), Item>(
                Method::GET,
                format!("{}api/item/{}", self.entry_point, item_id),
                &(),
            )
        }

        pub fn delete_item(&self, item_id: &str) -> Result<RowAffected> {
            self.request::<(), RowAffected>(
                Method::DELETE,
                format!("{}api/item/{}", self.entry_point, item_id),
                &(),
            )
        }

        pub fn insert_feedback(&self, feedback: &Vec<Feedback>) -> Result<RowAffected> {
            self.request(
                Method::POST,
                format!("{}api/feedback", self.entry_point),
                feedback,
            )
        }

        pub fn delete_feedback(&self, user_id: &str, item_id: &str) -> Result<RowAffected> {
            self.request::<(), RowAffected>(
                Method::DELETE,
                format!("{}api/feedback/{}/{}", self.entry_point, user_id, item_id),
                &(),
            )
        }

        pub fn list_feedback(&self, user_id: &str, feedback_type: &str) -> Result<Vec<Feedback>> {
            self.request::<(), Vec<Feedback>>(
                Method::GET,
                format!(
                    "{}api/user/{}/feedback/{}",
                    self.entry_point, user_id, feedback_type
                ),
                &(),
            )
        }

        pub fn get_item_neighbors(&self, item_id: &str) -> Result<Vec<Score>> {
            self.request::<(), Vec<Score>>(
                Method::GET,
                format!("{}api/item/{}/neighbors", self.entry_point, item_id),
                &(),
            )
        }

        pub fn get_recommend(
            &self,
            user_id: &str,
            options: RecommendOptions,
        ) -> Result<Vec<String>> {
            let mut url = format!("{}api/recommend/{}", self.entry_point, user_id);
            if options.n > 0 {
                url = format!("{}?n={}", url, options.n);
            }
            self.request::<(), Vec<String>>(Method::GET, url, &())
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
            if response.status() == StatusCode::OK {
                let r: RetType = serde_json::from_str(response.text()?.as_str())?;
                Ok(r)
            } else {
                Err(Error::Api {
                    status_code: response.status(),
                    message: response.text()?,
                })
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json::json;
        use serial_test::serial;

        const ENTRY_POINT: &str = "http://127.0.0.1:8088/";
        const API_KEY: &str = "zhenghaoz";

        #[test]
        #[serial]
        fn test_users() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let user = User {
                user_id: "2000".into(),
                labels: json!({
                    "gender": "M",
                    "occupation": "engineer"
                }),
                comment: "zhenghaoz".into(),
            };
            let r = client.insert_user(&user)?;
            assert_eq!(r.row_affected, 1);
            let resp = client.get_user("2000")?;
            assert_eq!(user, resp);

            let r = client.delete_user("2000")?;
            assert_eq!(r.row_affected, 1);
            match client.get_user("2000") {
                Ok(_) => panic!("Expected error"),
                Err(Error::Api { status_code, .. }) => {
                    assert_eq!(status_code, StatusCode::NOT_FOUND);
                }
                Err(e) => panic!("Expected API error, got {:?}", e),
            }
            Ok(())
        }

        #[test]
        #[serial]
        fn test_items() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let item = Item {
                item_id: "2000".into(),
                is_hidden: true,
                labels: json!({
                    "embedding": [0.1, 0.2, 0.3]
                }),
                categories: vec!["Comedy".into(), "Animation".into()],
                timestamp: "2022-11-20T13:55:27Z".into(),
                comment: "Minions (2015)".into(),
            };
            let r = client.insert_item(&item)?;
            assert_eq!(r.row_affected, 1);
            let resp = client.get_item("2000")?;
            assert_eq!(item, resp);

            let r = client.delete_item("2000")?;
            assert_eq!(r.row_affected, 1);
            match client.get_item("2000") {
                Ok(_) => panic!("Expected error"),
                Err(Error::Api { status_code, .. }) => {
                    assert_eq!(status_code, StatusCode::NOT_FOUND);
                }
                Err(e) => panic!("Expected API error, got {:?}", e),
            }
            Ok(())
        }

        #[test]
        #[serial]
        fn test_feedback() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            client.insert_user(&User {
                user_id: "2000".into(),
                labels: json!({}),
                comment: "".into(),
            })?;

            let feedbacks = vec![
                Feedback {
                    feedback_type: "watch".into(),
                    user_id: "2000".into(),
                    item_id: "1".into(),
                    value: 1.0,
                    timestamp: "2022-11-20T13:55:27Z".into(),
                },
                Feedback {
                    feedback_type: "watch".into(),
                    user_id: "2000".into(),
                    item_id: "1060".into(),
                    value: 2.0,
                    timestamp: "2022-11-20T13:55:27Z".into(),
                },
                Feedback {
                    feedback_type: "watch".into(),
                    user_id: "2000".into(),
                    item_id: "11".into(),
                    value: 3.0,
                    timestamp: "2022-11-20T13:55:27Z".into(),
                },
            ];
            for fb in &feedbacks {
                client.delete_feedback(&fb.user_id, &fb.item_id)?;
            }
            let r = client.insert_feedback(&feedbacks)?;
            assert_eq!(r.row_affected, 3);
            let user_feedback = client.list_feedback("2000", "watch")?;
            assert_eq!(feedbacks, user_feedback);

            let r = client.delete_feedback("2000", "1")?;
            assert_eq!(r.row_affected, 1);
            let user_feedback = client.list_feedback("2000", "watch")?;
            assert_eq!(
                vec![feedbacks[1].clone(), feedbacks[2].clone()],
                user_feedback
            );
            Ok(())
        }

        #[test]
        #[serial]
        fn test_item_to_item() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            let scores = client.get_item_neighbors("1")?;
            assert_eq!(scores[0].id, "1060".to_string());
            assert_eq!(scores[1].id, "404".to_string());
            assert_eq!(scores[2].id, "1219".to_string());
            Ok(())
        }

        #[test]
        #[serial]
        fn test_recommend() -> Result<()> {
            let client = Gorse::new(ENTRY_POINT, API_KEY);
            client.insert_user(&User {
                user_id: "3000".into(),
                labels: json!({}),
                comment: "".into(),
            })?;
            let items = client.get_recommend("3000", RecommendOptions { n: 3 })?;
            assert_eq!(
                items,
                vec!["315".to_string(), "1432".to_string(), "918".to_string()]
            );
            Ok(())
        }
    }
}
