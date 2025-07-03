use anchor_lang::prelude::*;


#[error_code]
pub enum DroxError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient balance of SOL")]
    InsufficientBalanceSol,
    #[msg("Insufficient balance of MSOL")]
    InsufficientBalanceMsol,
}