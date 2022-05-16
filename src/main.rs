extern crate colored;

mod up;

use colored::*;
use reqwest::{Client, Method, header};
use std::env;
use std::io::{stdin, stdout, Write};

use up::{AccountResponse, PingResponse, TransactionResponse, UP_API_BASE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(match env::var("UP_API_TOKEN") {
        Ok(val) => {
            let client_builder = Client::builder();
            let mut headers = header::HeaderMap::new();
            headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}", val)).unwrap());

            let client = client_builder.default_headers(headers).build().unwrap();

            let path = format!("{}/util/ping", UP_API_BASE);
            let req = client.request(Method::GET, &path);
            let resp: PingResponse = req.send().await?.json().await?;
            if resp.meta.status_emoji.trim() == "⚡️" {
                println!("✅ Logged In Successfully");
                repl(&client).await?;
            } else {
                println!("API not connected");
            }
        },
        Err(_) => println!("UP_API_TOKEN not found - Visit https://api.up.com.au/getting_started to obtain an Up API token"),
    })
}

async fn get_accounts(client: &Client) -> Result<Vec<up::AccountData>, Box<dyn std::error::Error>> {
    Ok(client
        .request(Method::GET, &format!("{}/accounts", UP_API_BASE))
        .send()
        .await?
        .json::<AccountResponse>()
        .await?
        .data
    )
}

async fn get_transactions(client: &Client, size: u8) -> Result<Vec<up::TransactionData>, Box<dyn std::error::Error>> {
    Ok(client
        .request(Method::GET, &format!("{}/transactions?page[size]={}", UP_API_BASE, size))
        .send()
        .await?
        .json::<TransactionResponse>()
        .await?
        .data
    )
}

fn print_transaction(transaction: &up::TransactionData) {
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

async fn repl(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
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
        match args[0] {
            "exit" | "quit" => break,
            "balance" | "accounts" => {
                for acc in get_accounts(&client).await? { println!(
                        "{}: ${}",
                        acc.attributes.display_name, acc.attributes.balance.value
                    );
                }
            },
            "transactions" => {
                let mut size: u8 = 10;
                if args.len() >= 2 {
                    size = args[1].parse::<u8>().unwrap();
                }
                for transaction in get_transactions(&client, size).await? {
                    print_transaction(&transaction);
                }
            },
            _ => print_help()
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
