#![allow(unexpected_cfgs)]

// Import Anchor framework and all instruction modules
use anchor_lang::prelude::*;
use instructions::{deposit_sol::*, liquid_unstake::*, order_unstake::*, claim::*};

// Error definitions for custom program errors
pub mod error;
/// All program instructions are defined in this module
pub mod instructions;

// Constants such as important public keys
pub mod constants;

// Declare the program ID for this Anchor program
// This must match the program ID in Anchor.toml
declare_id!("Bfjb9g895pAyGfuFGgkf1JcUzDeKdxYk2sDJC76RZ89r");

#[program]
/// Main program module for drox_project
pub mod drox_project {
    use super::*;

    /// Deposit SOL and receive mSOL via Marinade CPI
    ///
    /// # Arguments
    /// * `ctx` - Context containing all required accounts
    /// * `lamports` - Amount of SOL to deposit
    pub fn deposit(ctx: Context<DepositSol>, lamports: u64) -> Result<()> {
        ctx.accounts.process(lamports)
    }

    /// Liquid unstake mSOL for SOL via Marinade CPI
    ///
    /// # Arguments
    /// * `ctx` - Context containing all required accounts
    /// * `msol_amount` - Amount of mSOL to unstake
    pub fn liquid_unstake(ctx: Context<LiquidUnstakeSol>, msol_amount: u64) -> Result<()> {
        ctx.accounts.process(msol_amount)
    }

    /// Order an unstake ticket for delayed unstaking via Marinade CPI
    ///
    /// # Arguments
    /// * `ctx` - Context containing all required accounts
    /// * `msol_amount` - Amount of mSOL to order for unstake
    pub fn order_unstake(ctx: Context<OrderUnstakeSol>, msol_amount: u64) -> Result<()> {
         // Get the bump for the new_ticket_account PDA
         let bump = ctx.bumps.new_ticket_account;
         ctx.accounts.process(msol_amount, bump)
    }

    /// Claim SOL from a completed unstake ticket via Marinade CPI
    ///
    /// # Arguments
    /// * `ctx` - Context containing all required accounts
    pub fn claim(ctx: Context<ClaimSol>) -> Result<()> {
         // Get the bump for the ticket_account PDA
         let bump = ctx.bumps.ticket_account;
         ctx.accounts.process(bump)
    }
}

