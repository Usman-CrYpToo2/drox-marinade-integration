use crate::error::DroxError::{InsufficientBalanceMsol, InvalidAmount};
use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_cpi::cpi::accounts::LiquidUnstake;
use marinade_cpi::cpi::liquid_unstake;


#[derive(Accounts)]
pub struct LiquidUnstakeSol<'info> {
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
          #[account(mut)]
          pub treasury_msol_account: AccountInfo<'info>,
          #[account(
            mut,
            associated_token::mint = msol_mint,
            associated_token::authority = get_msol_from_authority
        )]
          pub get_msol_from: Account<'info, TokenAccount>,
          #[account(mut)]
          pub get_msol_from_authority: Signer<'info>, 
          #[account(mut)]
          pub transfer_sol_to: SystemAccount<'info>,
          pub system_program: Program<'info, System>,
          pub token_program: Program<'info, Token>,
          pub associated_token_program: Program<'info, AssociatedToken>,
      
          // accounts added are: Marinade main program ID, referral_state, partner token account
          /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
          #[account(address = MARINADE_ID_DEVNET)]
          pub marinade_finance_program: AccountInfo<'info>,

    }

    impl<'info> LiquidUnstakeSol<'info> {
        pub fn process(&mut self, msol_amount: u64) -> Result<()> {
        if msol_amount == 0 {
            return Err(InvalidAmount.into());
        }

        let user_msol_account_balance = self.get_msol_from.amount;
        if user_msol_account_balance < msol_amount {
            return Err(InsufficientBalanceMsol.into());
        }

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

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);
        liquid_unstake(cpi_ctx, msol_amount)
            

        }

    }
