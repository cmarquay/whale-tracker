use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EtherscanResponse {
    pub status: String,
    pub message: String,
    pub result: Vec<Transaction>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub value: String,
    #[serde(rename = "timeStamp")]
    pub time_stamp: String,
}
