extern crate colored;

mod model;
mod tally_income;
mod up;

use colored::*;
use reqwest::{header, Client, Method};
use std::env;
use std::io::{stdin, stdout, Write};
use std::process;
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

/// Parsed transaction request data
struct TransactionRequest {
    size: usize,
    category: Option<String>,
    account: Option<String>,
}

/// Parse transaction command arguments
fn parse_transaction_args(args: &[&str]) -> TransactionRequest {
    let mut size: usize = 10;
    let mut category: Option<String> = None;
    let mut account: Option<String> = None;
    let mut i = 0;

    while i < args.len() {
        match args[i] {
            "-c" | "--category" => {
                if i + 1 < args.len() {
                    category = Some(args[i + 1].to_string());
                    i += 2;
                } else {
                    eprintln!("--category requires a value");
                    process::exit(1);
                }
            }
            "-a" | "--account" => {
                if i + 1 < args.len() {
                    account = Some(args[i + 1].to_string());
                    i += 2;
                } else {
                    eprintln!("--account requires a value");
                    process::exit(1);
                }
            }
            _ => {
                size = args[i].parse::<usize>().unwrap_or(size);
                i += 1;
            }
        }
    }

    TransactionRequest { size, category, account }
}

/// Fetch and display transactions based on request parameters
async fn display_transactions(client: &Client, req: TransactionRequest) -> Result<(), Box<dyn std::error::Error>> {
    // Resolve account name to ID if needed
    let account_id = match &req.account {
        Some(name) => {
            let accounts = get_accounts(&client).await?;
            let found = accounts.iter().find(|a| {
                a.id == *name
                    || a.attributes.display_name.to_lowercase() == name.to_lowercase()
            });
            match found {
                Some(acc) => Some(acc.id.clone()),
                None => {
                    eprintln!("Account '{}' not found. Available accounts:", name);
                    for acc in accounts {
                        eprintln!("  - {}", acc.attributes.display_name);
                    }
                    process::exit(1);
                }
            }
        }
        None => None,
    };

    let transactions = match (&account_id, &req.category) {
        (Some(acc), cat) => {
            get_transactions_by_account(&client, acc, cat.as_deref(), req.size).await?
        }
        (None, Some(cat)) => get_transactions_by_category(&client, cat, req.size).await?,
        (None, None) => get_transactions(&client, req.size).await?,
    };

    for transaction in transactions {
        print_transaction(&transaction);
    }

    Ok(())
}

async fn eval(args: Vec<&str>, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
        return Ok(());
    }

    match args[0] {
        "balance" | "accounts" => {
            for acc in get_accounts(&client).await? {
                println!(
                    "{}: ${} ({})",
                    acc.attributes.display_name, acc.attributes.balance.value, acc.id
                );
            }
        }
        "transactions" => {
            let transaction_request = parse_transaction_args(&args[1..]);
            display_transactions(client, transaction_request).await?;
        }
        "categories" => {
            let categories = get_categories(&client).await?;
            // Group by parent
            let mut parents: Vec<&model::CategoryData> = categories
                .iter()
                .filter(|c| c.relationships.parent.data.is_none())
                .collect();
            parents.sort_by(|a, b| a.attributes.name.cmp(&b.attributes.name));

            for parent in parents {
                println!("{} ({})", parent.attributes.name.bold(), parent.id);
                let mut children: Vec<&model::CategoryData> = categories
                    .iter()
                    .filter(|c| {
                        c.relationships
                            .parent
                            .data
                            .as_ref()
                            .map(|p| p.id == parent.id)
                            .unwrap_or(false)
                    })
                    .collect();
                children.sort_by(|a, b| a.attributes.name.cmp(&b.attributes.name));
                for child in children {
                    println!("  {} ({})", child.attributes.name, child.id);
                }
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
        if args.is_empty() {
            continue;
        }
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
     - balance                          (prints all account balances with IDs)
     - transactions [COUNT] [OPTIONS]   (show last COUNT transactions, defaults to 10)
         -c, --category CAT             (filter by category ID)
         -a, --account NAME             (filter by account name or ID)
     - categories                       (list all transaction categories)
     - tally_income                     (counts all transactions related to income)
     - exit                             (quits the app)"
    );
}
