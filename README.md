# gorse-rs

[![CI](https://github.com/gorse-io/gorse-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/gorse-io/gorse-rs/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/gorse_rs)](https://crates.io/crates/gorse_rs/)

Rust SDK for gorse recommender system

## Install

```toml
[dependencies]
gorse_rs = "0.4.0"
```

## Usage

- Use async client:

```rust
use gorse_rs::{Feedback, Gorse};

let client = Gorse::new("http://127.0.0.1:8087", "api_key");

let feedback = vec![
    Feedback::new("star", "bob", "vuejs:vue", "2022-02-24"),
    Feedback::new("star", "bob", "d3:d3", "2022-02-25"),
    Feedback::new("star", "bob", "dogfalo:materialize", "2022-02-26"),
    Feedback::new("star", "bob", "mozilla:pdf.js", "2022-02-27"),
    Feedback::new("star", "bob", "moment:moment", "2022-02-28")
];
client.insert_feedback(&feedback);

client.get_recommend("100");
```

- Use blocking client:

```rust
use gorse_rs::Feedback;
use gorse_rs::blocking::Gorse;

let client = Gorse::new("http://127.0.0.1:8087", "api_key");

let feedback = vec![
    Feedback::new("star", "bob", "vuejs:vue", "2022-02-24"),
    Feedback::new("star", "bob", "d3:d3", "2022-02-25"),
    Feedback::new("star", "bob", "dogfalo:materialize", "2022-02-26"),
    Feedback::new("star", "bob", "mozilla:pdf.js", "2022-02-27"),
    Feedback::new("star", "bob", "moment:moment", "2022-02-28")
];
client.insert_feedback(&feedback).await;

client.get_recommend("100").await;
```
