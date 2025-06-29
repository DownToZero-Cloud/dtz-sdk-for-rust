# DownToZero Cloud SDK

[![Latest Version](https://img.shields.io/crates/v/dtz.svg)](https://crates.io/crates/dtz)

A base crate for the DownToZero Cloud SDK

## Exposed functionality

The `dtz`-crate only exposes the `dtz-config` crate which covers the API client and authentication requirements.

All service specific functionality is exposed through features.

### Features

* billing
* containers
* containerregistry
* core
* identity
* full (contains all features at once)
* objectstore
* observability
* rss2mail

## Examples

Retrieving the current Context.

```
[dependencies]
tokio = { version = "1", features = ["full] }
dtz = { version = "*", features = ["core"] }
```

```rust
#[tokio::main]
use std::str::FromStr;

async fn main() {
    let config = dtz::Configuration {
        api_key: Some("some api key".to_string()),
        ..Default::default()
    };
    let ctx_id = "context-00000000";
    let result = dtz::core::apis::default_api::get_context(&config, ctx_id)
        .await
        .unwrap();
    println!("result: {result:?}");
}
```
