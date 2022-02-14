extern crate colored;

mod up;

use colored::*;
use std::env;
use std::io::{stdin, stdout, Write};

use up::{AccountResponse, PingResponse, TransactionResponse, UP_API_BASE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(match env::var("UP_API_TOKEN") {
        Ok(val) => {
            let client = reqwest::Client::new();
            let path = format!("{}/util/ping", UP_API_BASE);
            let req = client.request(reqwest::Method::GET, &path)
                .bearer_auth(&val);
            let resp: PingResponse = req.send().await?.json().await?;
            // If the response is parsed than we have a valid token.
            if resp.meta.status_emoji.trim() == "⚡️" {
                println!("✅ Logged In Successfully");
                repl(&val).await?;
            } else {
                println!("API not connected");
            }
        },
        Err(_) => println!("UP_API_TOKEN not found - Visit https://api.up.com.au/getting_started to obtain an Up API token"),
    })
}

async fn repl(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        // Step 1: Prompt
        print!("⚡ ");
        stdout().flush()?;
        // Step 2: Read
        let mut command = String::new();
        stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        let args: Vec<&str> = command.trim().split_whitespace().collect();
        // Step 3: Evaluate
        if args.len() == 0 {
            print_help();
        } else if args[0] == "exit" || args[0] == "quit" {
            break;
        } else if args[0] == "balance" || args[0] == "accounts" {
            // Call Up API to retreive all account balances
            let client = reqwest::Client::new();
            let resp: AccountResponse = client
                .request(reqwest::Method::GET, &format!("{}/accounts", UP_API_BASE))
                .bearer_auth(api_key)
                .send()
                .await?
                .json()
                .await?;
            // Now loop over accounts
            for acc in resp.data {
                println!(
                    "{}: ${}",
                    acc.attributes.display_name, acc.attributes.balance.value
                );
            }
        } else if args[0] == "transactions" {
            let mut size: u8 = 10;
            if args.len() >= 2 {
                size = args[1].parse::<u8>().unwrap();
            }
            // Call Up API to get last 10 transactions
            let client = reqwest::Client::new();
            let resp: TransactionResponse = client
                .request(
                    reqwest::Method::GET,
                    &format!("{}/transactions?page[size]={}", UP_API_BASE, size),
                )
                .bearer_auth(api_key)
                .send()
                .await?
                .json()
                .await?;
            // Now print all transactions
            for transaction in resp.data {
                let verb = if transaction.attributes.amount.value_in_base_units < 0 {
                    "to"
                } else {
                    "from"
                };
                let coloured_transaction = if transaction.attributes.amount.value_in_base_units < 0
                {
                    transaction.attributes.amount.value.red()
                } else {
                    transaction.attributes.amount.value.green()
                };
                println!(
                    "{} {} {}",
                    coloured_transaction, verb, transaction.attributes.description
                );
            }
        } else {
            print_help();
        }
    }
    println!("Thanks for using Up Banking CLI - Have a nice day!");
    Ok(())
}

fn print_help() {
    println!(
        "Up Banking CLI Help
    Commands:
     - balance              (prints all account balances)
     - transactions [COUNT] (show last COUNT transactions, defaults to 10)
     - exit                 (quits the app)"
    );
}
