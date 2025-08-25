use crate::state::{Presale, SolVault};
use anchor_lang::prelude::*;
use crate::errors::Errs;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct InitializePresale<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_mint: Account<'info,Mint>,
    #[account(
        init, 
        payer = authority,
        seeds= [b"presale", authority.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space= 8 + Presale::INIT_SPACE,
    )]
    pub presale: Account<'info, Presale>,
    #[account(
        init,
        payer= authority,
        associated_token::mint=token_mint,
        associated_token::authority=presale,
    )]
    pub token_vault_presale: Account<'info, TokenAccount>,
    #[account(
        init,
        payer= authority,
        token::mint = token_mint,
        token::authority = presale,
        seeds = [b"lp-vault", presale.key().as_ref()],
        bump,
    )]
    pub token_vault_lp: Account<'info, TokenAccount>,
    #[account(
        init,
        payer= authority, 
        seeds=[b"sol-vault", presale.key().as_ref()],
        bump,
        space = 8 + SolVault::INIT_SPACE,
    )]
    pub sol_vault: Account<'info,SolVault>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info,Token>,
    pub system_program: Program<'info, System>,
}

impl <'info> InitializePresale <'info>{
    pub fn init_presale(&mut self,seed: u64, hard_cap_lamports:u64,target_presale_tokens:u64,end_time_unix:i64,bumps:&InitializePresaleBumps)-> Result<()>{
        require!(end_time_unix> Clock::get()?.unix_timestamp, Errs::BadTiming);
        require!(hard_cap_lamports>0,Errs::ZeroCap);
        require!(target_presale_tokens>0,Errs::ZeroTarget);
        require!(target_presale_tokens % 2 == 0, Errs::TargetMustBeEven);
        self.presale.set_inner(Presale {
            seed,
            bump:bumps.presale,
            authority: self.authority.key(),
            token_mint: self.token_mint.key(),
            token_vault_presale:self.token_vault_presale.key(),
            token_vault_lp: self.token_vault_lp.key(),
            sol_vault: self.sol_vault.key(),
            lp_token_mint: Pubkey::default(),
            lp_token_vault: Pubkey::default(),
            hard_cap_lamports,
            target_presale_tokens,
            start_time_unix: Clock::get()?.unix_timestamp,
            end_time_unix,
            tokens_deposited_presale:0,
            tokens_deposited_lp: 0,
            sol_raised_lamports: 0,
            is_finalized: false, 
            is_canceled: false 
        });
        Ok(())
    }
}
