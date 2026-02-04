use crate::model::{self, AccountResponse, CategoryResponse, TransactionResponse, UP_API_BASE};
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
    size: usize,
) -> Result<Vec<model::TransactionData>, Box<dyn Error>> {
    let page_size = std::cmp::min(size, 100);
    let mut response = client
        .request(
            Method::GET,
            &format!("{}/transactions?page[size]={}", UP_API_BASE, page_size),
        )
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?;

    let mut transactions = response.data;

    // Paginate if we need more than 100
    while transactions.len() < size {
        match response.links.next {
            Some(url) => {
                response = client
                    .request(Method::GET, url)
                    .send()
                    .await?
                    .json::<TransactionResponse>()
                    .await?;
                transactions.append(&mut response.data);
            }
            None => break,
        }
    }

    // Truncate to exact requested size
    transactions.truncate(size);
    Ok(transactions)
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

pub async fn get_categories(client: &Client) -> Result<Vec<model::CategoryData>, Box<dyn Error>> {
    Ok(client
        .request(Method::GET, &format!("{}/categories", UP_API_BASE))
        .send()
        .await?
        .json::<CategoryResponse>()
        .await?
        .data)
}

pub async fn get_transactions_by_category(
    client: &Client,
    category: &str,
    size: usize,
) -> Result<Vec<model::TransactionData>, Box<dyn Error>> {
    let page_size = std::cmp::min(size, 100);
    let mut response = client
        .request(
            Method::GET,
            &format!(
                "{}/transactions?filter[category]={}&page[size]={}",
                UP_API_BASE, category, page_size
            ),
        )
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?;

    let mut transactions = response.data;

    while transactions.len() < size {
        match response.links.next {
            Some(url) => {
                response = client
                    .request(Method::GET, url)
                    .send()
                    .await?
                    .json::<TransactionResponse>()
                    .await?;
                transactions.append(&mut response.data);
            }
            None => break,
        }
    }

    transactions.truncate(size);
    Ok(transactions)
}

pub async fn get_transactions_by_account(
    client: &Client,
    account_id: &str,
    category: Option<&str>,
    size: usize,
) -> Result<Vec<model::TransactionData>, Box<dyn Error>> {
    let page_size = std::cmp::min(size, 100);
    let url = match category {
        Some(cat) => format!(
            "{}/accounts/{}/transactions?filter[category]={}&page[size]={}",
            UP_API_BASE, account_id, cat, page_size
        ),
        None => format!(
            "{}/accounts/{}/transactions?page[size]={}",
            UP_API_BASE, account_id, page_size
        ),
    };

    let mut response = client
        .request(Method::GET, &url)
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?;

    let mut transactions = response.data;

    while transactions.len() < size {
        match response.links.next {
            Some(url) => {
                response = client
                    .request(Method::GET, url)
                    .send()
                    .await?
                    .json::<TransactionResponse>()
                    .await?;
                transactions.append(&mut response.data);
            }
            None => break,
        }
    }

    transactions.truncate(size);
    Ok(transactions)
}
