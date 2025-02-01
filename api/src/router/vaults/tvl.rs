use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::Arc,
};

use anchor_lang::prelude::Pubkey;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use borsh::BorshDeserialize;
use jito_bytemuck::Discriminator;
use jito_vault_client::{accounts::Vault, programs::JITO_VAULT_ID};
use solana_account_decoder::UiAccountEncoding;
use solana_rpc_client_api::{
    config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};

use crate::{error::JitoRestakingApiError, router::RouterState};

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

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tvl {
    /// Vault Pubkey
    vault_pubkey: String,

    /// Supported Token (JitoSOL, JTO...)
    supported_mint: String,

    /// The amount of tokens deposited in Vault
    native_unit_tvl: f64,

    /// Supported mint token symbol
    native_unit_symbol: String,

    /// The amount of tokens deposited in Vault in USD
    usd_tvl: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CoinData {
    decimals: u8,
    price: f64,
    symbol: String,
    timestamp: f64,
}

#[derive(Debug, serde::Deserialize)]
struct CoinResponse {
    coins: HashMap<String, CoinData>,
}

pub async fn get_tvls(State(state): State<Arc<RouterState>>) -> crate::Result<impl IntoResponse> {
    let vault_accounts = state
        .rpc_client
        .get_program_accounts_with_config(
            &JITO_VAULT_ID,
            RpcProgramAccountsConfig {
                filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
                    0,
                    MemcmpEncodedBytes::Bytes(vec![jito_vault_core::vault::Vault::DISCRIMINATOR]),
                ))]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: None,
                    min_context_slot: None,
                },
                with_context: None,
            },
        )
        .await?;

    let mut vrt_pubkeys = Vec::new();
    let metadata_program = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();
    let vrt_metadata_pubkeys: Vec<Pubkey> = vault_accounts
        .iter()
        .map(|(_, vault)| {
            let vault = Vault::deserialize(&mut vault.data.as_slice()).unwrap();
            vrt_pubkeys.push(vault.vrt_mint);
            Pubkey::find_program_address(
                &[
                    "metadata".as_bytes(),
                    metadata_program.as_ref(),
                    vault.vrt_mint.as_ref(),
                ],
                &metadata_program,
            )
            .0
        })
        .collect();

    let vrt_metadata_accs = state
        .rpc_client
        .get_multiple_accounts(&vrt_metadata_pubkeys)
        .await
        .unwrap();

    println!("{:?}", vrt_metadata_accs);

    let metadatas: HashMap<Pubkey, Metadata> = vrt_metadata_accs
        .iter()
        .enumerate()
        .filter_map(|(index, metadata_acc)| {
            if let Some(acc) = metadata_acc {
                // let mut slice = &acc.data.as_slice();
                if let Ok(data_v2) = Metadata::deserialize(&mut acc.data.as_slice()) {
                    if let Some(vrt_pubkey) = vrt_pubkeys.get(index) {
                        return Some((*vrt_pubkey, data_v2));
                    }
                }
            }

            None
        })
        .collect();

    let st_pubkeys: HashSet<String> = vault_accounts
        .iter()
        .map(|(_, vault)| {
            let vault = Vault::deserialize(&mut vault.data.as_slice()).unwrap();
            vault.supported_mint.to_string()
        })
        .collect();
    let st_pubkeys: Vec<String> = st_pubkeys.into_iter().collect();

    let base_url = String::from("https://coins.llama.fi/prices/current/solana:");
    let url = format!("{base_url}{}", st_pubkeys.join(",solana:"));

    let response: CoinResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    let mut tvls = Vec::new();
    for (vault_pubkey, vault) in vault_accounts {
        let vault = Vault::deserialize(&mut vault.data.as_slice()).unwrap();

        let key = format!("solana:{}", vault.supported_mint);
        let (native_unit_symbol, price_usd, decimals) =
            response
                .coins
                .get(&key)
                .map_or(("", 0_f64, 0_u8), |coin_data| {
                    (
                        coin_data.symbol.as_str(),
                        coin_data.price,
                        coin_data.decimals,
                    )
                });

        let decimal_factor = 10u64.pow(decimals as u32) as f64;
        let native_unit_tvl = vault.tokens_deposited as f64 / decimal_factor;
        let (symbol, url) = match metadatas.get(&vault.vrt_mint) {
            Some(metadata) => {
                let inner_metadata: serde_json::Value =
                    reqwest::get(&metadata.uri).await?.json().await?;
                (
                    inner_metadata["symbol"].to_string(),
                    inner_metadata["image"].to_string(),
                )
            }
            None => ("".to_string(), "".to_string()),
        };

        tvls.push(crate::state::tvl::Tvl::new(
            vault_pubkey.to_string(),
            vault.supported_mint.to_string(),
            native_unit_symbol.to_string(),
            native_unit_tvl,
            vault.vrt_mint.to_string(),
            &symbol,
            &url,
            native_unit_tvl * price_usd,
        ));
    }

    tvls.sort_by(|a, b| b.usd_tvl.total_cmp(&a.usd_tvl));

    Ok(Json(tvls))
}

pub async fn get_tvl(
    State(state): State<Arc<RouterState>>,
    Path(vault_pubkey): Path<String>,
) -> crate::Result<impl IntoResponse> {
    let vault_pubkey = Pubkey::from_str(&vault_pubkey)?;

    let account = state.rpc_client.get_account(&vault_pubkey).await?;
    let vault = Vault::deserialize(&mut account.data.as_slice()).map_err(|e| {
        tracing::warn!("error deserializing Vault: {:?}", e);
        JitoRestakingApiError::AnchorError(e.into())
    })?;

    let url = format!(
        "https://coins.llama.fi/prices/current/solana:{}",
        vault.supported_mint,
    );
    let response: CoinResponse = reqwest::get(url).await.unwrap().json().await.unwrap();

    let key = format!("solana:{}", vault.supported_mint);
    let (native_unit_symbol, price_usd, decimals) =
        response
            .coins
            .get(&key)
            .map_or(("", 0_f64, 0_u8), |coin_data| {
                (
                    coin_data.symbol.as_str(),
                    coin_data.price,
                    coin_data.decimals,
                )
            });

    let decimal_factor = 10u64.pow(decimals as u32) as f64;
    let native_unit_tvl = vault.tokens_deposited as f64 / decimal_factor;
    let tvl = Tvl {
        vault_pubkey: vault_pubkey.to_string(),
        supported_mint: vault.supported_mint.to_string(),
        native_unit_tvl,
        native_unit_symbol: native_unit_symbol.to_string(),
        usd_tvl: native_unit_tvl * price_usd,
    };

    Ok(Json(tvl))
}
