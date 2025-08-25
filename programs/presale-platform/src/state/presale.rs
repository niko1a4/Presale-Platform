use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct SolVault {}
#[account]
#[derive(InitSpace)]
pub struct Presale {
    pub seed: u64,         //to allow multiple presales
    pub bump: u8,          // PDA bump
    pub authority: Pubkey, //project owner

    //==assets & vaults==
    pub token_mint: Pubkey,
    pub token_vault_presale: Pubkey,
    pub token_vault_lp: Pubkey,
    pub sol_vault: Pubkey,

    //lp token mint (unknown at init time, set during finalize_presale, Pubkey::default() until then)
    pub lp_token_mint: Pubkey,
    //ATA(owner = presale PDA, mint-lp_token_mint) created at finalize_presale
    pub lp_token_vault: Pubkey,

    //==economic params==
    pub hard_cap_lamports: u64,     //global hard cap in lamports
    pub target_presale_tokens: u64, //how many tokens goes to presale

    //==timing==
    pub start_time_unix: i64, //when contributions start
    pub end_time_unix: i64,   //when contributions end

    // ==totals==
    pub tokens_deposited_presale: u64, //how many total tokens did dev team transfer for presale
    pub tokens_deposited_lp: u64,      //how many tokens did dev team trasnfer for lp
    pub sol_raised_lamports: u64,      //how much sol was raised in total

    pub is_finalized: bool, //is the presale finalized or still going
    pub is_canceled: bool,  //is the presale canceled
}
