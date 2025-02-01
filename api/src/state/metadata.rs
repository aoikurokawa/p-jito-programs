use std::time::Duration;

use anchor_lang::prelude::Pubkey;
use borsh::BorshDeserialize;
use reqwest::Client;

#[derive(Clone, BorshDeserialize, Debug, PartialEq, Eq)]
pub struct Metadata {
    pub key: u8,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<u8>>,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
}

impl Metadata {
    pub async fn fetch_inner_metadata(&self) -> (String, String) {
        let client = Client::new();
        let timeout = Duration::from_secs(2);

        let metadata_uri = self.uri.trim();
        if let Err(_e) = reqwest::Url::parse(metadata_uri) {
            return ("".to_string(), "".to_string());
        }

        let response = client
            .get(metadata_uri.to_string())
            .timeout(timeout)
            .send()
            .await;

        match response {
            Ok(resp) => {
                if let Ok(inner_metadata) = resp.json::<serde_json::Value>().await {
                    let symbol = inner_metadata["symbol"].as_str().unwrap_or("").to_string();
                    let image = inner_metadata["image"].as_str().unwrap_or("").to_string();

                    return (symbol, image);
                }
            }
            Err(e) => {
                println!("HTTP request failed or timed out: {:?}", e);
            }
        }

        ("".to_string(), "".to_string())
    }
}
