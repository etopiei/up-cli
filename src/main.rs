extern crate colored;

mod up;

use std::env;
use std::io::{Write, stdout, stdin};
use colored::*;

use up::{PingResponse, AccountResponse, TransactionResponse, UP_API_BASE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(match env::var("UP_API_TOKEN") {
        Ok(val) => {
            let client = reqwest::Client::new();
            let path = format!("{}/util/ping", UP_API_BASE);
            let req = client.request(reqwest::Method::GET, &path)
                .header(reqwest::header::AUTHORIZATION, &val);
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
        stdin().read_line(&mut command).expect("Failed to read command");
        // Step 3: Evaluate
        if command.trim() == "exit" || command.trim() == "quit" {
            break;
        } else if command.trim() == "balance" || command.trim() == "accounts" {
            // Call Up API to retreive all account balances
            let client = reqwest::Client::new();
            let resp: AccountResponse = client.request(reqwest::Method::GET, &format!("{}/accounts", UP_API_BASE))
                .header(reqwest::header::AUTHORIZATION, api_key)
                .send().await?
                .json().await?;
            // Now loop over accounts
            for acc in resp.data {
                println!("{}: ${}", acc.attributes.display_name, acc.attributes.balance.value);
            }
        } else if command.trim() == "transactions" {
            // Call Up API to get last 10 transactions
            let client = reqwest::Client::new();
            let resp: TransactionResponse = client.request(reqwest::Method::GET, &format!("{}/transactions?page[size]=10", UP_API_BASE))
                .header(reqwest::header::AUTHORIZATION, api_key)
                .send().await?
                .json().await?;
            // Now print all transactions
            for transaction in resp.data {
                let verb = if transaction.attributes.amount.value_in_base_units < 0 { "to" } else { "from" };
                let coloured_transaction = if transaction.attributes.amount.value_in_base_units < 0 { transaction.attributes.amount.value.red() } else { transaction.attributes.amount.value.green() };
                println!("{} {} {}", coloured_transaction, verb, transaction.attributes.description);
            }
        } else {
            print_help();
        }
    }
    println!("Thanks for using Up Banking CLI - Have a nice day!");
    Ok(())
}

fn print_help() {
    println!("Up Banking CLI Help
    Commands:
     - balance (prints all account balances)
     - transactions (show last 10 transactions)
     - exit (quits the app)");
}
