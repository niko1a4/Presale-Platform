use crate::{state::{Presale, SolVault}};
use anchor_lang::prelude::*;
use crate::errors::Errs;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct DepositTokens<'info>{
     #[account(mut)]
    pub depositor: Signer<'info>,
    pub token_mint: Account<'info,Mint>,
    #[account( 
        mut,
        seeds= [b"presale", presale.authority.key().as_ref(),presale.seed.to_le_bytes().as_ref()],
        bump= presale.bump,
        has_one = token_mint,
        has_one = token_vault_presale,
        has_one = token_vault_lp,
    )]
    pub presale: Account<'info, Presale>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=presale,
    )]
    pub token_vault_presale: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=depositor,
    )]
    pub token_ata: Account<'info, TokenAccount>,
     #[account(
        mut,
        seeds = [b"lp-vault", presale.key().as_ref()],
        bump,
        constraint = token_vault_lp.mint==token_mint.key() @ Errs::InvalidLpMint,
        constraint = token_vault_lp.owner==presale.key() @ Errs::InvalidOwner,
    )]
    pub token_vault_lp: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
}
impl <'info> DepositTokens<'info>{
    pub fn deposit_tokens(&mut self, amount_of_tokens: u64)-> Result<()>{
        require!(amount_of_tokens>0, Errs::BadPresaleDepositAmount);
        require!(!self.presale.is_finalized && !self.presale.is_canceled, Errs::Closed);
        require_keys_eq!(self.presale.authority, self.depositor.key(), Errs::Unauthorized);
        require!(self.presale.tokens_deposited_presale == 0 && self.presale.tokens_deposited_lp == 0, Errs::InvalidState);
        let target = self.presale.target_presale_tokens;
        require!((target + (target/2))==amount_of_tokens,Errs::BadPresaleDepositAmount);
        let amount_to_token_vault = self.presale.target_presale_tokens;
        let amount_to_lp_vault= amount_of_tokens-amount_to_token_vault;
        let program= self.token_program.to_account_info();
        let accounts = Transfer{
            from: self.token_ata.to_account_info(),
            to: self.token_vault_presale.to_account_info(),
            authority: self.depositor.to_account_info(),
        };
        let cpi_ctx= CpiContext::new(program, accounts);
        transfer(cpi_ctx, amount_to_token_vault)?;
        let program= self.token_program.to_account_info();
        let accounts= Transfer{
            from: self.token_ata.to_account_info(),
            to: self.token_vault_lp.to_account_info(),
            authority: self.depositor.to_account_info(),
        };
        let cpi_ctx= CpiContext::new(program, accounts);
        transfer(cpi_ctx, amount_to_lp_vault)?;

        self.presale.tokens_deposited_presale= amount_to_token_vault;
        self.presale.tokens_deposited_lp=amount_to_lp_vault;
        Ok(())
    }
}