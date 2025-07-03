use crate::constants::*;
use anchor_lang::prelude::*;
use marinade_cpi::cpi::accounts::Claim;
use marinade_cpi::cpi::claim;
use marinade_cpi::TicketAccountData;

/// Accounts context for the claim instruction.
/// This struct defines all accounts required to claim SOL from a completed Marinade unstake ticket.
#[derive(Accounts)]
#[instruction(ticket_id: u64)]
pub struct ClaimSol<'info> {
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    // Marinade reserve PDA (system account)
    #[account(mut)]
    pub reserve_pda: SystemAccount<'info>,
    // Marinade ticket account (PDA, checked by seeds and owner)
    #[account(
        mut,
        seeds = [
            b"ticket",
            state.key().as_ref(),
            transfer_sol_to.key().as_ref(),
            ticket_id.to_le_bytes().as_ref()        
        ],
        bump ,
        owner = marinade_finance_program.key()
    )]
    pub ticket_account: Account<'info, TicketAccountData>,
    // The beneficiary of the ticket (must match ticket_account.beneficiary)
    #[account(
        mut,
        address = ticket_account.beneficiary
    )]
    pub transfer_sol_to: SystemAccount<'info>,
    // Clock sysvar for time-based logic
    pub clock: Sysvar<'info, Clock>,
    // System program
    pub system_program: Program<'info, System>,
    /// CHECK: Marinade main program. Validated by address.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,
}

impl<'info> ClaimSol<'info> {
    /// Processes a claim for SOL from a completed Marinade unstake ticket.
    ///
    /// # Arguments
    /// * `bump` - The bump seed for the ticket_account PDA
    pub fn process(&mut self, ticket_id: u64 ) -> Result<()> {
        // Prepare CPI context for Marinade claim
        let cpi_program: AccountInfo<'_> = self.marinade_finance_program.to_account_info();
        let cpi_account =  Claim {
            state: self.state.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            ticket_account: self.ticket_account.to_account_info(),
            transfer_sol_to: self.transfer_sol_to.to_account_info(),
            clock: self.clock.to_account_info(),
            system_program: self.system_program.to_account_info()
        };
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);  
        // Perform the CPI call to Marinade's claim instruction
        claim(cpi_ctx)
    }
}
