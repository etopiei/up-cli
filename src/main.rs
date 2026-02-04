extern crate colored;

mod model;
mod tally_income;
mod up;

use colored::*;
use reqwest::{header, Client, Method};
use std::env;
use std::io::{stdin, stdout, Write};
use model::{PingResponse, UP_API_BASE};
use up::*;

use crate::tally_income::tally_income;

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
                // Here decide if we need to go to repl or not.
                // i.e. is there a command to execute as an arg.
                let args_owned: Vec<String> = env::args().collect();
                let args: Vec<&str> = args_owned.iter().map(String::as_str).collect();
                if args.len() == 1 {
                    println!("✅ Logged In Successfully");
                    repl(&client).await?;
                } else {
                    eval((&args[1..]).to_vec(), &client).await?;
                }
            } else {
                println!("API not connected");
            }
        },
        Err(_) => println!("UP_API_TOKEN not found - Visit https://api.up.com.au/getting_started to obtain an Up API token"),
    })
}

fn print_transaction(transaction: &model::TransactionData) {
    let verb = if transaction.attributes.amount.value_in_base_units < 0 {
        "to"
    } else {
        "from"
    };
    let coloured_transaction = if transaction.attributes.amount.value_in_base_units < 0 {
        transaction.attributes.amount.value.red()
    } else {
        transaction.attributes.amount.value.green()
    };
    println!(
        "{} {} {}",
        coloured_transaction, verb, transaction.attributes.description
    );
}

async fn eval(args: Vec<&str>, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    if args.len() == 0 {
        return Ok(());
    }

    // Step 3: Evaluate
    match args[0] {
        "balance" | "accounts" => {
            for acc in get_accounts(&client).await? {
                println!(
                    "{}: ${}",
                    acc.attributes.display_name, acc.attributes.balance.value
                );
            }
        }
        "transactions" => {
            let mut size: u8 = 10;
            if args.len() >= 2 {
                size = args[1].parse::<u8>().unwrap();
            }
            for transaction in get_transactions(&client, size).await? {
                print_transaction(&transaction);
            }
        }
        "tally_income" => {
            let tally = tally_income(&client).await?;

            for (raw_text, amount) in tally {
                println!("{}: ${}", raw_text, amount);
            }
        }
        _ => print_help(),
    }

    Ok(())
}

async fn repl(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("⚡ ");
        stdout().flush()?;

        let mut command = String::new();
        stdin()
            .read_line(&mut command)
            .expect("Failed to read command");
        let args: Vec<&str> = command.trim().split_whitespace().collect();
        match args[0] {
            "exit" | "quit" => break,
            _ => eval(args, client).await?,
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
     - tally_income (counts all transactions related to income)
     - exit                 (quits the app)"
    );
}
