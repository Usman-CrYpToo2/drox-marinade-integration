use crate::error::DroxError::{InsufficientBalanceMsol, InvalidAmount};
use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_cpi::cpi::accounts::LiquidUnstake;
use marinade_cpi::cpi::liquid_unstake;

/// Accounts context for the liquid_unstake instruction.
/// This struct defines all accounts required to liquid unstake mSOL for SOL via Marinade CPI.
#[derive(Accounts)]
pub struct LiquidUnstakeSol<'info> {
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
    /// CHECK: Marinade treasury mSOL account. Validated by CPI program.
    #[account(mut)]
    pub treasury_msol_account: AccountInfo<'info>,
    // User's mSOL token account to burn from
    #[account(
        mut,
        associated_token::mint = msol_mint,
        associated_token::authority = get_msol_from_authority
    )]
    pub get_msol_from: Account<'info, TokenAccount>,
    // User authority for burning mSOL
    #[account(mut)]
    pub get_msol_from_authority: Signer<'info>, 
    // User's SOL account to receive unstaked SOL
    #[account(mut)]
    pub transfer_sol_to: SystemAccount<'info>,
    // System, token, and associated token programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Marinade main program. Validated by address.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,
}

impl<'info> LiquidUnstakeSol<'info> {
    /// Processes a liquid unstake of mSOL for SOL via Marinade CPI.
    ///
    /// # Arguments
    /// * `msol_amount` - Amount of mSOL to unstake
    pub fn process(&mut self, msol_amount: u64) -> Result<()> {
        // Check for valid amount
        if msol_amount == 0 {
            return Err(InvalidAmount.into());
        }
        // Check user has enough mSOL
        let user_msol_account_balance = self.get_msol_from.amount;
        if user_msol_account_balance < msol_amount {
            return Err(InsufficientBalanceMsol.into());
        }

        // Prepare CPI context for Marinade liquid_unstake
        let cpi_program = self.marinade_finance_program.to_account_info();
        let cpi_account = LiquidUnstake {
            state: self.state.to_account_info(),
            msol_mint : self.msol_mint.to_account_info(),
            liq_pool_msol_leg: self.liq_pool_msol_leg.to_account_info(),
            liq_pool_sol_leg_pda: self.liq_pool_sol_leg_pda.to_account_info(),
            treasury_msol_account: self.treasury_msol_account.to_account_info(),
            get_msol_from: self.get_msol_from.to_account_info(),
            get_msol_from_authority: self.get_msol_from_authority.to_account_info(),
            transfer_sol_to: self.transfer_sol_to.to_account_info(),
            system_program: self.system_program.to_account_info(),
            token_program: self.token_program.to_account_info(),
        };

        // Perform the CPI call to Marinade's liquid_unstake instruction
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        liquid_unstake(cpi_ctx, msol_amount)
    }
}
