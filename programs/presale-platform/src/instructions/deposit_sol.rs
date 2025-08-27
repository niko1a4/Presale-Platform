use crate::{state::{Presale, SolVault}};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program,system_instruction::transfer};
use crate::errors::Errs;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct DepositSol<'info>{
     #[account(mut)]
    pub depositor: Signer<'info>,
    pub token_mint: Account<'info,Mint>,
    #[account( 
        mut,
        seeds= [b"presale", presale.authority.key().as_ref(),presale.seed.to_le_bytes().as_ref()],
        bump= presale.bump,
        has_one = token_mint,
    )]
    pub presale: Account<'info, Presale>,
    #[account(
        mut,
        seeds=[b"sol-vault", presale.key().as_ref()],
        bump,
    )]
    pub sol_vault: Account<'info, SolVault>,
    pub system_program: Program<'info, System>,
}
impl<'info> DepositSol<'info>{
    pub fn deposit_sol(&mut self, amount_lamports: u64)-> Result<()>{
        let now= Clock::get()?.unix_timestamp;
        require!(now>self.presale.start_time_unix, Errs::PresaleNotYetStarted);
        require!(now<=self.presale.end_time_unix, Errs::PresaleEnded);
        require!(self.presale.is_finalized==false, Errs::AlreadyFinalized);
        require!(amount_lamports>0, Errs::InvalidAmount);
        let current_fill_rate = self.presale.sol_raised_lamports;
        let hard_cap = self.presale.hard_cap_lamports;
        if current_fill_rate >= hard_cap{
            return err!(Errs::ExceedsHardCap);
        }
        let remaining = hard_cap-current_fill_rate;
        let take = remaining.min(amount_lamports); //returns min of the two numbers
        if take==0{
            return err!(Errs::ExceedsHardCap);
        }
        
        let ix = transfer(
            &self.depositor.key(),
            &self.sol_vault.key(),
            take
        );
        program::invoke(
            &ix, 
        &[
            self.depositor.to_account_info(),
            self.sol_vault.to_account_info()
        ],
        )?;
        self.presale.sol_raised_lamports= self.presale.sol_raised_lamports.checked_add(take)
        .ok_or(Errs::Overflow)?;
        Ok(())
    }
}