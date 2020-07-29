use std::env;
use std::io::{Write, stdout, stdin};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(match env::var("UP_API_TOKEN") {
        Ok(val) => {
            let client = reqwest::Client::new();
            let req = client.request(reqwest::Method::GET, "https://api.up.com.au/api/v1/util/ping").header(reqwest::header::AUTHORIZATION, val);
            let resp = req.send().await?.text().await?;
            println!("{:#?}", resp);
            // Check here for success response and drop into REPL
            println!("✅ Logged In Successfully");
            // Probably parse the token to the REPL here
            repl().await?;
        },
        Err(_) => println!("UP_API_TOKEN not found."),
    })
}

async fn repl() -> Result<(), Box<dyn std::error::Error>> {
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
        } else if command.trim() == "balance" {
            // Call Up API to retreive all account balances
        }
        // Step 4: Print Result
    }
    println!("Thanks for using Up Banking CLI");
    Ok(())
}
