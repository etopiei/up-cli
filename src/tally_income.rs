use crate::up::get_all_transactions;
use itertools::Itertools;
use reqwest::Client;
use std::error::Error;

/// Groups all of the incoming transactions by their raw text and sums the amount
// Method chaining for the win! Love Rust
pub async fn tally_income(client: &Client) -> Result<Vec<(String, f64)>, Box<dyn Error>> {
    Ok(get_all_transactions(client)
        .await?
        .iter()
        .filter(|&transaction| {
            transaction.attributes.amount.value_in_base_units > 0
                && transaction.attributes.raw_text.is_some()
        })
        .sorted_by(|&a, &b| {
            a.attributes
                .raw_text
                .as_ref()
                .unwrap()
                .cmp(&b.attributes.raw_text.as_ref().unwrap())
        })
        .group_by(|&transaction| transaction.attributes.raw_text.clone().unwrap())
        .into_iter()
        .map(|(raw_text, transactions)| {
            (
                raw_text,
                transactions.fold(0.0, |accumulator, transaction| {
                    accumulator + transaction.attributes.amount.value_in_base_units as f64 / 100.0
                }),
            )
        })
        .collect::<Vec<(std::string::String, f64)>>())
}
