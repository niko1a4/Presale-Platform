// errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum Errs {
    // ---------- General ----------
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Math overflow")]
    Overflow,
    #[msg("Math underflow")]
    Underflow,
    #[msg("Invalid state")]
    InvalidState,

    // ---------- Init / Config ----------
    #[msg("target_presale_tokens must be > 0")]
    ZeroTarget,
    #[msg("hard_cap_lamports must be > 0")]
    ZeroCap,
    #[msg("Invalid timing: start_time must be before end_time")]
    BadTiming,
    #[msg("target_presale_tokens must be even for 50/50 LP bucket")]
    TargetMustBeEven,

    // ---------- Accounts / Seeds ----------
    #[msg("Invalid token mint")]
    InvalidMint,
    #[msg("Invalid presale token vault (ATA)")]
    InvalidPresaleVault,
    #[msg("Invalid LP bucket token vault (PDA)")]
    InvalidLpBucketVault,
    #[msg("Invalid SOL vault PDA")]
    InvalidSolVault,
    #[msg("Invalid LP token mint")]
    InvalidLpMint,
    #[msg("Invalid LP token ATA")]
    InvalidLpVault,
    #[msg("Account owner mismatch")]
    InvalidOwner,
    #[msg("Program id mismatch")]
    InvalidProgram,

    // ---------- Lifecycle ----------
    #[msg("Presale is not active")]
    NotActive,
    #[msg("Presale not started")]
    NotStarted,
    #[msg("Presale closed")]
    Closed,
    #[msg("Presale already finalized")]
    AlreadyFinalized,
    #[msg("Presale is cancelled")]
    Cancelled,
    #[msg("Presale not finalized yet")]
    NotFinalized,

    // ---------- Deposits ----------
    #[msg("Token buckets are not fully deposited")]
    TokensNotDeposited,
    #[msg("Bad presale bucket deposit amount")]
    BadPresaleDepositAmount,
    #[msg("Bad LP bucket deposit amount")]
    BadLpDepositAmount,
    #[msg("Exceeds hard cap")]
    ExceedsHardCap,
    #[msg("Contribution is zero or below minimum")]
    BadContributionAmount,

    // ---------- Finalize / LP ----------
    #[msg("Finalize allowed only after end time or when hard cap is reached")]
    FinalizeWindow,
    #[msg("Presale or LP bucket incomplete")]
    BucketsIncomplete,
    #[msg("Insufficient SOL to add liquidity")]
    InsufficientSolForLp,
    #[msg("Insufficient tokens to add liquidity")]
    InsufficientTokensForLp,
    #[msg("Add liquidity CPI failed")]
    AddLiquidityFailed,

    // ---------- Claim / Refund ----------
    #[msg("Nothing to claim")]
    NothingToClaim,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Nothing to refund")]
    NothingToRefund,
    #[msg("Refunds are not enabled")]
    NotRefundable,
}
