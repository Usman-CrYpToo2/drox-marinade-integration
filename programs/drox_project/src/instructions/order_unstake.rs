use crate::error::DroxError::{InsufficientBalanceMsol, InvalidAmount};
use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_cpi::cpi::accounts::OrderUnstake;
use marinade_cpi::cpi::order_unstake;
use marinade_cpi::TicketAccountData;


#[derive(Accounts)]
pub struct OrderUnstakeSol<'info> {
     /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    #[account(mut, address = MSOL_MINT)]
    pub msol_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = msol_mint,
        associated_token::authority = burn_msol_authority
    )]
    pub burn_msol_from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub burn_msol_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [
            state.key().as_ref(),
            burn_msol_authority.key().as_ref(),
            b"ticket"
        ],
        bump ,
        space = 8 + std::mem::size_of::<TicketAccountData>(),
        owner = marinade_finance_program.key()
    )]
    pub new_ticket_account:Account<'info, TicketAccountData>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,

}

impl<'info> OrderUnstakeSol<'info> {
    pub fn process(&mut self, msol_amount: u64, bump: u8) -> Result<()> {
        if msol_amount == 0 {
            return Err(InvalidAmount.into());
        }

        let user_msol_account_balance = self.burn_msol_from.amount;
        if user_msol_account_balance < msol_amount {
            return Err(InsufficientBalanceMsol.into());
        };

        let cpi_program: AccountInfo<'_> = self.marinade_finance_program.to_account_info();

        let cpi_account = OrderUnstake {
            state: self.state.to_account_info(),
            msol_mint: self.msol_mint.to_account_info(),
            burn_msol_from: self.burn_msol_from.to_account_info(),
            burn_msol_authority: self.burn_msol_authority.to_account_info(),
            new_ticket_account: self.new_ticket_account.to_account_info(),
            clock: self.clock.to_account_info(),
            rent: self.rent.to_account_info(),
            token_program: self.token_program.to_account_info()
        };
        
        let seeds_for_new_ticket_account: &[&[u8]] = &[
            self.state.key.as_ref(),
            self.burn_msol_authority.key.as_ref(),
            b"ticket",
            &[bump],
        ];

        let signer_seeds: &[&[&[u8]]] = &[seeds_for_new_ticket_account];
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_account,
            signer_seeds,
        );

        order_unstake(cpi_ctx, msol_amount)
    


    }
}