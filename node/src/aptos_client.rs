use url::Url;
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.testnet.aptoslabs.com"),
    )
    .unwrap()
});