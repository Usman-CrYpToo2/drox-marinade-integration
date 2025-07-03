use crate::error::DroxError::{InsufficientBalanceMsol, InvalidAmount};
use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};
use marinade_cpi::cpi::accounts::OrderUnstake;
use marinade_cpi::cpi::order_unstake;
use marinade_cpi::TicketAccountData;

/// Accounts context for the order_unstake instruction.
/// This struct defines all accounts required to order an unstake ticket via Marinade CPI.
#[derive(Accounts)]
#[instruction( msol_amount: u64, ticket_id: u64)]
pub struct OrderUnstakeSol<'info> {
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    // Marinade mSOL mint (checked by address)
    #[account(mut, address = MSOL_MINT)]
    pub msol_mint: Account<'info, Mint>,
    // User's mSOL token account to burn from
    #[account(
        mut,
        associated_token::mint = msol_mint,
        associated_token::authority = burn_msol_authority
    )]
    pub burn_msol_from: Account<'info, TokenAccount>,
    // User authority for burning mSOL
    #[account(mut)]
    pub burn_msol_authority: Signer<'info>,
    // Payer for the new ticket account
    #[account(mut)]
    pub payer: Signer<'info>,
    // Marinade ticket account (PDA, checked by seeds and owner)
    #[account(
        init_if_needed,
        payer = payer,
        seeds = [
            b"ticket",
            state.key().as_ref(),
            burn_msol_authority.key().as_ref(),
            ticket_id.to_le_bytes().as_ref()        
        ],
        bump ,
        space = 8 + std::mem::size_of::<TicketAccountData>(),
        owner = marinade_finance_program.key()
    )]
    pub new_ticket_account: Account<'info, TicketAccountData>,
    // Clock and rent sysvars
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    // System, token, and associated token programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Marinade main program. Validated by address.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,
}

impl<'info> OrderUnstakeSol<'info> {
    /// Processes an order to unstake mSOL and create a Marinade ticket account.
    ///
    /// # Arguments
    /// * `msol_amount` - Amount of mSOL to order for unstake
    /// * `bump` - The bump seed for the new_ticket_account PDA
    pub fn process(&mut self, msol_amount: u64,ticket_id: u64 ) -> Result<()> {
        // Check for valid amount
        if msol_amount == 0 {
            return Err(InvalidAmount.into());
        }
        // Check user has enough mSOL
        let user_msol_account_balance = self.burn_msol_from.amount;
        if user_msol_account_balance < msol_amount {
            return Err(InsufficientBalanceMsol.into());
        };

        // Prepare CPI context for Marinade order_unstake
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
    
          let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        // Perform the CPI call to Marinade's order_unstake instruction
          order_unstake(cpi_ctx, msol_amount)
        
    }
}