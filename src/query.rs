use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CursorQuery {
    #[serde(rename = "n")]
    pub number: Option<i32>,
    pub cursor: Option<String>,
}

impl CursorQuery {
    pub fn new() -> Self {
        CursorQuery {
            number: None,
            cursor: None,
        }
    }

    pub fn number(mut self, number: i32) -> Self {
        self.number = Some(number);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OffsetQuery {
    #[serde(rename = "n")]
    pub number: Option<i32>,
    pub offset: Option<i32>,
}

impl OffsetQuery {
    pub fn new() -> Self {
        OffsetQuery {
            number: None,
            offset: None,
        }
    }

    pub fn number(mut self, number: i32) -> Self {
        self.number = Some(number);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdQuery {
    #[serde(rename = "user-id")]
    pub user_id: Option<String>,
    #[serde(rename = "n")]
    pub number: Option<i32>,
    pub offset: Option<i32>,
}

impl UserIdQuery {
    pub fn new() -> Self {
        UserIdQuery {
            user_id: None,
            number: None,
            offset: None,
        }
    }

    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn number(mut self, number: i32) -> Self {
        self.number = Some(number);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WriteBackQuery {
    #[serde(rename = "write-back-type")]
    pub write_back_type: Option<String>,
    #[serde(rename = "write-back-delay")]
    pub write_back_delay: Option<String>,
    #[serde(rename = "n")]
    pub number: Option<i32>,
    pub offset: Option<i32>,
}

impl WriteBackQuery {
    pub fn new() -> Self {
        WriteBackQuery {
            write_back_type: None,
            write_back_delay: None,
            number: None,
            offset: None,
        }
    }

    pub fn write_back_type(mut self, write_back_type: impl Into<String>) -> Self {
        self.write_back_type = Some(write_back_type.into());
        self
    }

    pub fn write_back_delay(mut self, write_back_delay: impl Into<String>) -> Self {
        self.write_back_delay = Some(write_back_delay.into());
        self
    }

    pub fn number(mut self, number: i32) -> Self {
        self.number = Some(number);
        self
    }

    pub fn offset(mut self, offset: i32) -> Self {
        self.offset = Some(offset);
        self
    }
}
