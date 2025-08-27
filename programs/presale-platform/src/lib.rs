#![allow(deprecated, unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("B6MgxnXF4ZhUp3c9Y9SJR5stk1PsZi2a4rfErhNRrWoG");
mod errors;
mod instructions;
use instructions::*;
mod state;
#[program]
pub mod presale_platform {
    use crate::instructions::InitializePresale;

    use super::*;

    pub fn initialize(
        ctx: Context<InitializePresale>,
        seed: u64,
        hard_cap_lamports: u64,
        target_presale_tokens: u64,
        end_time_unix: i64,
    ) -> Result<()> {
        ctx.accounts.init_presale(
            seed,
            hard_cap_lamports,
            target_presale_tokens,
            end_time_unix,
            &ctx.bumps,
        )?;
        Ok(())
    }
    pub fn deposit_tokens(ctx: Context<DepositTokens>, amount_of_tokens: u64) -> Result<()> {
        ctx.accounts.deposit_tokens(amount_of_tokens)?;
        Ok(())
    }
}
