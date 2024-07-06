mod r#async;
pub mod blocking;
pub mod query;

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::{error, fmt};

pub use r#async::*;
use reqwest::{Method, StatusCode};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Users {
    #[serde(rename = "Users")]
    users: Vec<User>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Labels")]
    pub labels: Vec<String>,
}

impl User {
    pub fn new(user_id: impl Into<String>, labels: Vec<impl Into<String>>) -> Self {
        User {
            user_id: user_id.into(),
            labels: labels.into_iter().map(|label| label.into()).collect(),
        }
    }
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

impl Item {
    pub fn new(
        item_id: impl Into<String>,
        labels: Vec<impl Into<String>>,
        categories: Vec<impl Into<String>>,
        timestamp: impl Into<String>,
    ) -> Self {
        Item {
            item_id: item_id.into(),
            is_hidden: false,
            labels: labels.into_iter().map(|label| label.into()).collect(),
            categories: categories
                .into_iter()
                .map(|category| category.into())
                .collect(),
            timestamp: timestamp.into(),
            comment: String::new(),
        }
    }

    pub fn is_hidden(mut self, is_hidden: bool) -> Self {
        self.is_hidden = is_hidden;
        self
    }

    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = comment.into();
        self
    }
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
    pub fn new(
        feedback_type: impl Into<String>,
        user_id: impl Into<String>,
        item_id: impl Into<String>,
        timestamp: impl Into<String>,
    ) -> Self {
        Feedback {
            feedback_type: feedback_type.into(),
            user_id: user_id.into(),
            item_id: item_id.into(),
            timestamp: timestamp.into(),
        }
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
