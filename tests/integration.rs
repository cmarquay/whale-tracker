use whale_tracker::etherscan::fetch_transactions;

#[tokio::test]
async fn test_fetch_transactions() {
    let addr = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    let txs = fetch_transactions(addr).await.unwrap();
    assert!(!txs.is_empty());
}
