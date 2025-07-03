use crate::error::DroxError::{InsufficientBalanceSol, InvalidAmount};
use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_cpi::cpi::accounts::Deposit;
use marinade_cpi::cpi::deposit;

#[derive(Accounts)]
pub struct DepositSol<'info> {
    // this part is equivalent to marinade-finance deposit instructions
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    #[account(mut, address = MSOL_MINT)]
    pub msol_mint: Account<'info, Mint>,
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub liq_pool_sol_leg_pda: AccountInfo<'info>,
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub liq_pool_msol_leg: AccountInfo<'info>,
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    pub liq_pool_msol_leg_authority: AccountInfo<'info>,
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub reserve_pda: AccountInfo<'info>,
    #[account(mut)]
    pub transfer_from: Signer<'info>,
    #[account(
        init_if_needed, 
        payer = transfer_from, 
        associated_token::mint = msol_mint,
        associated_token::authority = transfer_from
    )]
    pub mint_to: Account<'info, TokenAccount>,
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    pub msol_mint_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // accounts added are: Marinade main program ID, referral_state, partner token account
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,
}

impl<'info> DepositSol<'info> {
    pub fn process(&mut self, lamports: u64) -> Result<()> {
        msg!("enter Deposit::process {}", lamports);
        if lamports == 0 {
            return Err(InvalidAmount.into());
        }
        let transfer_from_lamports = self.transfer_from.lamports();
        if transfer_from_lamports < lamports {
            return Err(InsufficientBalanceSol.into());
        }

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

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        deposit(cpi_ctx, lamports)?;

        Ok(())
    }
}
