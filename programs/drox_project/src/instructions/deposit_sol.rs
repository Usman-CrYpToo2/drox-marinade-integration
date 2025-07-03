use crate::error::DroxError::{InsufficientBalanceSol, InvalidAmount};
use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_cpi::cpi::accounts::Deposit;
use marinade_cpi::cpi::deposit;

/// Accounts context for the deposit instruction.
/// This struct defines all accounts required to deposit SOL and receive mSOL via Marinade CPI.
#[derive(Accounts)]
pub struct DepositSol<'info> {
    // Marinade state account (unchecked)
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    // Marinade mSOL mint (checked by address)
    #[account(mut, address = MSOL_MINT)]
    pub msol_mint: Account<'info, Mint>,
    /// CHECK: Marinade liquidity pool SOL leg PDA. Validated by CPI program.
    #[account(mut)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    /// CHECK: Marinade liquidity pool mSOL leg. Validated by CPI program.
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,
    /// CHECK: Marinade liquidity pool mSOL leg authority. Validated by CPI program.
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,
    /// CHECK: Marinade reserve PDA. Validated by CPI program.
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    // User paying for the deposit
    #[account(mut)]
    pub transfer_from: Signer<'info>,
    // User's mSOL token account (created if needed)
    #[account(
        init_if_needed, 
        payer = transfer_from, 
        associated_token::mint = msol_mint,
        associated_token::authority = transfer_from
    )]
    pub mint_to: Account<'info, TokenAccount>,
    /// CHECK: Marinade mSOL mint authority. Validated by CPI program.
    pub msol_mint_authority: AccountInfo<'info>,
    // System, token, and associated token programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    // Marinade main program
    /// CHECK: Marinade main program. Validated by address.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,
}

impl<'info> DepositSol<'info> {
    /// Processes a deposit of SOL to Marinade and mints mSOL to the user.
    ///
    /// # Arguments
    /// * `lamports` - Amount of SOL to deposit
    pub fn process(&mut self, lamports: u64) -> Result<()> {
        msg!("enter Deposit::process {}", lamports);
        // Check for valid amount
        if lamports == 0 {
            return Err(InvalidAmount.into());
        }
        // Check user has enough SOL
        let transfer_from_lamports = self.transfer_from.lamports();
        if transfer_from_lamports < lamports {
            return Err(InsufficientBalanceSol.into());
        }

        // Prepare CPI context for Marinade deposit
        let cpi_program = self.marinade_finance_program.to_account_info();
        let cpi_accounts = Deposit {
            state: self.state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            liq_pool_msol_leg_authority: self.liq_pool_msol_leg_authority.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            transfer_from: self.transfer_from.to_account_info(),
            mint_to: self.mint_to.to_account_info(),
            msol_mint_authority: self.msol_mint_authority.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        // Perform the CPI call to Marinade's deposit instruction
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        deposit(cpi_ctx, lamports)?;

        Ok(())
    }
}
