#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use instructions::{deposit_sol::*, liquid_unstake::*, order_unstake::*, claim::*};

pub mod error;
///instructions
pub mod instructions;

pub mod constants;

declare_id!("Bfjb9g895pAyGfuFGgkf1JcUzDeKdxYk2sDJC76RZ89r");

#[program]
pub mod drox_project {

    use super::*;

    pub fn deposit(ctx: Context<DepositSol>, lamports: u64) -> Result<()> {
        ctx.accounts.process(lamports)
    }

    pub fn liquid_unstake(ctx: Context<LiquidUnstakeSol>, msol_amount: u64) -> Result<()> {
        ctx.accounts.process(msol_amount)
    }

    pub fn order_unstake(ctx: Context<OrderUnstakeSol>, msol_amount: u64) -> Result<()> {
         let bump = ctx.bumps.new_ticket_account;
         ctx.accounts.process(msol_amount, bump)
    }

    pub fn claim(ctx: Context<ClaimSol>) -> Result<()> {
         let bump = ctx.bumps.ticket_account;
         ctx.accounts.process(bump)
    }
}

