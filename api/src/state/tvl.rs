#[derive(serde::Serialize, serde::Deserialize)]
pub struct Tvl {
    /// Vault Pubkey
    vault_pubkey: String,

    /// Supported Token (JitoSOL, JTO...)
    supported_mint: String,

    /// Supported mint token symbol
    supported_mint_symbol: String,

    /// The amount of tokens deposited in Vault
    supported_mint_tvl: f64,

    /// Vault Recipt Token
    vrt_mint: String,

    /// Supported mint token symbol
    vrt_mint_symbol: String,

    /// Supported mint token symbol
    vrt_mint_url: String,

    /// The amount of tokens deposited in Vault in USD
    pub usd_tvl: f64,
}

impl Tvl {
    pub fn new(
        vault_pubkey: String,
        supported_mint: String,
        supported_mint_symbol: String,
        supported_mint_tvl: f64,
        vrt_mint: String,
        vrt_mint_symbol: &str,
        vrt_mint_url: &str,
        usd_tvl: f64,
    ) -> Self {
        Self {
            vault_pubkey,
            supported_mint,
            supported_mint_symbol,
            supported_mint_tvl,
            vrt_mint,
            vrt_mint_symbol: vrt_mint_symbol.to_string(),
            vrt_mint_url: vrt_mint_url.to_string(),
            usd_tvl,
        }
    }
}
