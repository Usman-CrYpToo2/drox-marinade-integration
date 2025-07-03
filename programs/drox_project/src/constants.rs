use anchor_lang::prelude::*;

/// The Marinade mSOL mint address (devnet/testnet/mainnet)
/// Used to verify and interact with the mSOL token throughout the program.
pub const MSOL_MINT: Pubkey = Pubkey::from_str_const("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");

/// The Marinade program ID (devnet/testnet/mainnet)
/// Used to verify and interact with the Marinade program for CPI calls.
pub const MARINADE_ID_DEVNET: Pubkey = Pubkey::from_str_const("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");
