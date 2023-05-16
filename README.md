# Up Banking CLI

![License Badge](https://img.shields.io/github/license/etopiei/up-cli)
![crates.io Badge](https://img.shields.io/crates/v/up-cli)

This is a simple command line program to interact with the Up Banking API.

## Installation

```bash
$ cargo install --force up-cli
```

## Usage

First you should get an API token from [Up](https://api.up.com.au/getting_started).

Then set it as an environment variable:

```bash
$ export UP_API_TOKEN="<token here>"
```

Then simply run:

```bash
$ up-cli
✅ Logged In Successfully
⚡ help
Up Banking CLI Help
    Commands:
     - balance (prints all account balances)
     - transactions (show last 10 transactions)
     - tally_income (counts all transactions related to income)
     - exit (quits the app)
⚡ exit
```

More features coming soon, feel free to help out with a PR!

