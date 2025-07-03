# drox coding assessment

A Solana Anchor program for interacting with Marinade Finance, enabling users to:
- Deposit SOL and receive mSOL
- Liquid unstake mSOL for SOL
- Order delayed unstake tickets
- Claim SOL from completed tickets

## Features
- **Deposit SOL:** Stake SOL via Marinade and receive mSOL
- **Liquid Unstake:** Instantly swap mSOL for SOL
- **Order Unstake:** Start a delayed unstake (ticket-based)
- **Claim:** Claim SOL from a completed unstake ticket

## Getting Started

### Prerequisites
- [Anchor](https://book.anchor-lang.com/chapter_2/installation.html) (v0.31.1 or later)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- Node.js & Yarn

### Install Dependencies
```sh
yarn install
```

### Build the Program
```sh
anchor build
```

### Run Tests
```sh
anchor test
```

## Example Transactions

Below are example transactions for each main function, executed on Solana devnet:

### 1. Deposit SOL
- **Function:** `deposit_sol`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/3tSVN3T93LWxdPwJ36xWFJHCLGmwmcHzSWCk4hosdpEkirVgmCjjqwjszRC53rfsvtPwretDMtkMY19PeBAHmo1K?cluster=devnet)

### 2. Liquid Unstake
- **Function:** `liquid_unstake`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/32N42gf1z8yooQ3LXSUkr3mgSgdXjFDxGJNrR4uhPqGZ8zRQ7UEDokEDyjxFXG1ZsWB8iAeJtvjFCNDtnvLin5hh?cluster=devnet)

### 3. Order Unstake
- **Function:** `order_unstake`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/5qjY3CocFfU91a8S5QKnDrGG3usNteNP3JBf5LYoLqbGz2vyMDmq4g9izpngajS4c2Ea2aX9og4NNK2Jo5AWwrnL?cluster=devnet)

### 4. Claim
- **Function:** `claim`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/3n1q6xe8pQ7PdUWfT2nYPHoFXNFyeyQnFrF7b4KuxK6H55MamQXjyHRL4Y1mHdoJG5EgTGvj59TjMxBPb3ehoShW?cluster=devnet)

## Project Structure
- `programs/` - Anchor Rust program
- `tests/` - TypeScript integration tests
- `migrations/` - Anchor deployment scripts
- `constants.rs` - Important public keys and addresses
- `error.rs` - Custom error codes

## License
MIT
