use serde::{Deserialize};

pub static UP_API_BASE: &str = "https://api.up.com.au/api/v1";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PingMetaData {
    id: String,
    pub status_emoji: String
}

#[derive(Deserialize)]
pub struct PingResponse {
    pub meta: PingMetaData
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    currency_code: String,
    pub value: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAttributes {
    pub display_name: String,
    account_type: String,
    pub balance: AccountBalance
}

#[derive(Deserialize)]
pub struct AccountData {
    id: String,
    r#type: String,
    pub attributes: AccountAttributes
}

#[derive(Deserialize)]
pub struct AccountResponse {
    pub data: Vec<AccountData>
    // links: AccountLinks
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAmount {
    currency_code: String,
    pub value: String,
    pub value_in_base_units: i32
}

#[derive(Deserialize)]
pub struct TransactionAttributes {
    pub amount: TransactionAmount,
    pub description: String,
    pub message: Option<String>
}

#[derive(Deserialize)]
pub struct TransactionData {
    id: String,
    r#type: String,
    pub attributes: TransactionAttributes
}

#[derive(Deserialize)]
pub struct TransactionResponse {
    pub data: Vec<TransactionData>
    // links: TransactionLinks
}