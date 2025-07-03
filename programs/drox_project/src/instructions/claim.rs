use crate::constants::*;
use anchor_lang::prelude::*;
use marinade_cpi::cpi::accounts::Claim;
use marinade_cpi::cpi::claim;
use marinade_cpi::TicketAccountData;

#[derive(Accounts)]
pub struct ClaimSol<'info> {
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(mut)]
    pub state: AccountInfo<'info>, // marinade state
    #[account(mut)]
    pub reserve_pda: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [
            state.key().as_ref(),
            transfer_sol_to.key().as_ref(),
            b"ticket"
        ],
        bump ,
        owner = marinade_finance_program.key()
    )]
    pub ticket_account: Account<'info, TicketAccountData>,
    #[account(
        mut,
        address = ticket_account.beneficiary
    )]
    pub transfer_sol_to: SystemAccount<'info>,

    pub clock: Sysvar<'info, Clock>,

    pub system_program: Program<'info, System>,
    
    /// CHECK: This is the Marinade state account. We trust the CPI program to validate it.
    #[account(address = MARINADE_ID_DEVNET)]
    pub marinade_finance_program: AccountInfo<'info>,

}

impl<'info> ClaimSol<'info> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.marinade_finance_program.to_account_info();
        
        let cpi_account =  Claim {
            state: self.state.to_account_info(),
            reserve_pda: self.reserve_pda.to_account_info(),
            ticket_account: self.ticket_account.to_account_info(),
            transfer_sol_to: self.transfer_sol_to.to_account_info(),
            clock: self.clock.to_account_info(),
            system_program: self.system_program.to_account_info()
        };
        
        let seeds_for_new_ticket_account: &[&[u8]] = &[
            self.state.key.as_ref(),
            self.transfer_sol_to.key.as_ref(),
            b"ticket",
            &[bump],
        ];

        let signer_seeds: &[&[&[u8]]] = &[seeds_for_new_ticket_account];
        let cpi_ctx = CpiContext::new_with_signer(
            cpi_program,
            cpi_account,
            signer_seeds,
        );
         
        claim(cpi_ctx)

    }
}
