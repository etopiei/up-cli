use crate::model::{self, AccountResponse, TransactionResponse, UP_API_BASE};
use reqwest::{Client, Method};
use std::error::Error;

pub async fn get_accounts(client: &Client) -> Result<Vec<model::AccountData>, Box<dyn Error>> {
    Ok(client
        .request(Method::GET, &format!("{}/accounts", UP_API_BASE))
        .send()
        .await?
        .json::<AccountResponse>()
        .await?
        .data)
}

pub async fn get_transactions(
    client: &Client,
    size: u8,
) -> Result<Vec<model::TransactionData>, Box<dyn Error>> {
    Ok(client
        .request(
            Method::GET,
            &format!("{}/transactions?page[size]={}", UP_API_BASE, size),
        )
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?
        .data)
}

/// Get all transactions from the Up API
pub async fn get_all_transactions(
    client: &Client,
) -> Result<Vec<model::TransactionData>, Box<dyn Error>> {
    let mut request = client
        .request(
            Method::GET,
            &format!("{}/transactions?page[size]={}", UP_API_BASE, 100),
        )
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?;

    let mut request_data = request.data;

    while let Some(url) = request.links.next {
        request = client
            .request(Method::GET, url)
            .send()
            .await?
            .json::<TransactionResponse>()
            .await?;

        request_data.append(&mut request.data);
    }

    Ok(request_data)
}
