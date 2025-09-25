# gorse-rs

[![CI](https://github.com/gorse-io/gorse-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/gorse-io/gorse-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/gorse_rs)](https://crates.io/crates/gorse_rs/)

Rust SDK for gorse recommender system

## Install

```toml
[dependencies]
gorse_rs = "0.5.0"
```

## Usage

- Use async client:

```rust
use gorse_rs::{Feedback, Gorse, User, Item, RecommendOptions};
use serde_json::json;

let client = Gorse::new("http://127.0.0.1:8087", "api_key");

// Insert a user
let user = User {
    user_id: "bob".into(),
    labels: json!({
        "gender": "M",
        "age": "30"
    }),
    comment: "a user".into(),
};
client.insert_user(&user).await?;

// Insert an item
let item = Item {
    item_id: "vuejs:vue".into(),
    is_hidden: false,
    labels: json!({
        "category": "frontend"
    }),
    categories: vec!["framework".into()],
    timestamp: "2022-11-20T13:55:27Z".into(),
    comment: "Vue.js framework".into(),
};
client.insert_item(&item).await?;

// Insert feedback
let feedback = vec![
    Feedback {
        feedback_type: "star".into(),
        user_id: "bob".into(),
        item_id: "vuejs:vue".into(),
        value: 1.0,
        timestamp: "2022-11-20T13:55:27Z".into(),
    },
    Feedback {
        feedback_type: "star".into(),
        user_id: "bob".into(),
        item_id: "d3:d3".into(),
        value: 1.0,
        timestamp: "2022-11-21T13:55:27Z".into(),
    },
    Feedback {
        feedback_type: "star".into(),
        user_id: "bob".into(),
        item_id: "dogfalo:materialize".into(),
        value: 1.0,
        timestamp: "2022-11-22T13:55:27Z".into(),
    },
];
client.insert_feedback(&feedback).await?;

// Get recommendation
let items = client.get_recommend("bob", RecommendOptions { n: 10 }).await?;
```

- Use blocking client:

```rust
use gorse_rs::{Feedback, User, Item, RecommendOptions};
use gorse_rs::blocking::Gorse;
use serde_json::json;

let client = Gorse::new("http://127.0.0.1:8087", "api_key");

// Insert a user
let user = User {
    user_id: "bob".into(),
    labels: json!({
        "gender": "M",
        "age": "30"
    }),
    comment: "a user".into(),
};
client.insert_user(&user)?;

// Insert an item
let item = Item {
    item_id: "vuejs:vue".into(),
    is_hidden: false,
    labels: json!({
        "category": "frontend"
    }),
    categories: vec!["framework".into()],
    timestamp: "2022-11-20T13:55:27Z".into(),
    comment: "Vue.js framework".into(),
};
client.insert_item(&item)?;

// Insert feedback
let feedback = vec![
    Feedback {
        feedback_type: "star".into(),
        user_id: "bob".into(),
        item_id: "vuejs:vue".into(),
        value: 1.0,
        timestamp: "2022-11-20T13:55:27Z".into(),
    },
    Feedback {
        feedback_type: "star".into(),
        user_id: "bob".into(),
        item_id: "d3:d3".into(),
        value: 1.0,
        timestamp: "2022-11-21T13:55:27Z".into(),
    },
    Feedback {
        feedback_type: "star".into(),
        user_id: "bob".into(),
        item_id: "dogfalo:materialize".into(),
        value: 1.0,
        timestamp: "2022-11-22T13:55:27Z".into(),
    },
];
client.insert_feedback(&feedback)?;

// Get recommendation
let items = client.get_recommend("bob", RecommendOptions { n: 10 })?;
```
