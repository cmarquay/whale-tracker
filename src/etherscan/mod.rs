use std::env;
use dotenv::dotenv;
use reqwest::Error;

mod types;
use types::{EtherscanResponse, Transaction};

pub async fn fetch_transactions(address: &str) -> Result<Vec<Transaction>, Error> {
    dotenv().ok();
    let api_key = env::var("ETHERSCAN_API_KEY").expect("API key not found");

    let url = format!(
        "https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=10&sort=desc&apikey={}",
        address,
        api_key
    );

    let resp = reqwest::get(&url).await?.json::<EtherscanResponse>().await?;
    Ok(resp.result)
}
