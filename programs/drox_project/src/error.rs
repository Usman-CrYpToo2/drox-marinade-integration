use anchor_lang::prelude::*;

/// Custom error codes for the drox_project program.
/// These errors are returned by instructions for invalid input, insufficient balances, etc.
#[error_code]
pub enum DroxError {
    /// Returned when the provided amount is zero or otherwise invalid.
    #[msg("Invalid amount")]
    InvalidAmount,
    /// Returned when the user does not have enough SOL to complete the operation.
    #[msg("Insufficient balance of SOL")]
    InsufficientBalanceSol,
    /// Returned when the user does not have enough mSOL to complete the operation.
    #[msg("Insufficient balance of MSOL")]
    InsufficientBalanceMsol,
}