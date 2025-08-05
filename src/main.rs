use whale_tracker::etherscan::fetch_transactions;

#[tokio::main]
async fn main() {
    let address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string();
    match fetch_transactions(&address).await {
        Ok(txs) => println!("{:#?}", txs),
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
