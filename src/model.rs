use serde::Deserialize;

pub static UP_API_BASE: &str = "https://api.up.com.au/api/v1";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PingMetaData {
    id: String,
    pub status_emoji: String,
}

#[derive(Deserialize)]
pub struct PingResponse {
    pub meta: PingMetaData,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AccountBalance {
    currency_code: String,
    pub value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct AccountAttributes {
    pub display_name: String,
    account_type: String,
    pub balance: AccountBalance,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct AccountData {
    pub id: String,
    r#type: String,
    pub attributes: AccountAttributes,
}

#[derive(Deserialize)]
pub struct AccountResponse {
    pub data: Vec<AccountData>, // links: AccountLinks
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TransactionAmount {
    currency_code: String,
    pub value: String,
    pub value_in_base_units: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAttributes {
    pub amount: TransactionAmount,
    pub description: String,
    pub message: Option<String>,
    pub raw_text: Option<String>,
}

#[derive(Deserialize)]
pub struct TransactionLinks {
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TransactionData {
    id: String,
    r#type: String,
    pub attributes: TransactionAttributes,
}

#[derive(Deserialize)]
pub struct TransactionResponse {
    pub data: Vec<TransactionData>, // links: TransactionLinks
    pub links: TransactionLinks,
}

// Category models
#[derive(Deserialize)]
pub struct CategoryAttributes {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CategoryParentData {
    pub id: String,
}

#[derive(Deserialize)]
pub struct CategoryParent {
    pub data: Option<CategoryParentData>,
}

#[derive(Deserialize)]
pub struct CategoryRelationships {
    pub parent: CategoryParent,
}

#[derive(Deserialize)]
pub struct CategoryData {
    pub id: String,
    pub attributes: CategoryAttributes,
    pub relationships: CategoryRelationships,
}

#[derive(Deserialize)]
pub struct CategoryResponse {
    pub data: Vec<CategoryData>,
}
