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
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/m5gUy4kjzqFipgFYE9L8CNGpM3DDBcLni6T5NSmtjegTA9fndokSV6JL6HKdKKMLK6wkP2LqUzSUqN6f9qJmCep?cluster=devnet)

### 2. Liquid Unstake
- **Function:** `liquid_unstake`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/2w6CWaNUH78aCiFCey4ibYCaVA4JC71S7ZER2LuNjJqjuvFLt13fKHUEtfSv9y7eUzJf63fbacDq92EATE9u7AiJ?cluster=devnet)

### 3. Order Unstake
- **Function:** `order_unstake`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/2PFkCVyzkJeWFaPUoUzGx5r1psEBBCEJbhSyiQfHzqaG3LimFNwJnk1pspAQuHzHSmS2acbMLkqpz5VEoxyPS1TS?cluster=devnet)

### 4. Claim
- **Function:** `claim`
- [View Transaction on Solana Explorer](https://explorer.solana.com/tx/3n1q6xe8pQ7PdUWfT2nYPHoFXNFyeyQnFrF7b4KuxK6H55MamQXjyHRL4Y1mHdoJG5EgTGvj59TjMxBPb3ehoShW?cluster=devnet)

## Project Structure

```
drox_project/
├── programs/
│   └── drox_project/
│       ├── src/
│       │   ├── instructions/
│       │   │   ├── claim.rs           # Claim SOL from completed ticket
│       │   │   ├── deposit_sol.rs     # Deposit SOL and receive mSOL
│       │   │   ├── liquid_unstake.rs  # Liquid unstake mSOL for SOL
│       │   │   ├── order_unstake.rs   # Order delayed unstake ticket
│       │   │   └── mod.rs             # Instruction module declarations
│       │   ├── constants.rs           # Important public keys and addresses
│       │   ├── error.rs               # Custom error codes
│       │   └── lib.rs                 # Program entrypoint and handlers
│       ├── Cargo.toml                 # Rust crate manifest
│       └── Xargo.toml                 # Xargo config (if needed)
├── tests/
│   ├── drox_project.ts                # TypeScript integration tests
│   └── constant.ts                    # Test constants (public keys, etc.)
├── migrations/
│   └── deploy.ts                      # Anchor deployment script
├── Anchor.toml                        # Anchor project config
├── Cargo.toml                         # Workspace manifest
├── package.json                       # Node.js dependencies
├── tsconfig.json                      # TypeScript config
├── yarn.lock                          # Yarn lockfile
└── README.md                          # Project documentation
```

- **programs/drox_project/src/instructions/**: All instruction handlers for the Anchor program
- **programs/drox_project/src/constants.rs**: Important public keys and addresses
- **programs/drox_project/src/error.rs**: Custom error codes for the program
- **programs/drox_project/src/lib.rs**: Main program entrypoint and instruction routing
- **tests/**: TypeScript integration tests and test constants
- **migrations/**: Anchor deployment scripts
- **Anchor.toml, Cargo.toml, package.json, etc.**: Project configuration files

## License
MIT
