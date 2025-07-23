use std::env;
use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EtherscanResponse {
    status: String,
    message: String,
    result: Vec<Transaction>,
}

#[derive(Debug, serde::Deserialize)]
struct Transaction {
    from: String,
    to: String,
    value: String,
    #[serde(rename = "timeStamp")]
    time_stamp: String, // Snake_case ici, mais on renomme pour Serde
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let api_key = env::var("ETHERSCAN_API_KEY").expect("API key not found");

    let address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"; // Exemple
    let url = format!(
        "https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock=0&endblock=99999999&page=1&offset=5&sort=desc&apikey={}",
        address, api_key
    );

    let resp = reqwest::get(&url).await?.json::<EtherscanResponse>().await?;

    println!("{:#?}", resp);

    Ok(())
}
