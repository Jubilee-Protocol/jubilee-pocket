use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Vault is paused")]
    Paused,
    #[msg("Invalid price feed")]
    InvalidPriceFeed,
    #[msg("Oracle price is stale")]
    StaleOracle,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Detailed error here")]
    GenericError,
    #[msg("Debt must be zero to withdraw")]
    DebtNotZero,
    #[msg("Cooldown period not yet met")]
    CooldownNotMet,
    #[msg("Loan is healthy, cannot liquidate")]
    LoanHealthy,
    #[msg("Commission too high")]
    CommissionTooHigh,
    #[msg("Guardian already whitelisted")]
    GuardianAlreadyWhitelisted,
    #[msg("Unauthorized: caller is not the loan owner")]
    Unauthorized,
    #[msg("Invalid mint authority: vault must be mint authority")]
    InvalidMintAuthority,
    #[msg("LTV too high: maximum allowed is 80%")]
    LTVTooHigh,
    #[msg("Invalid threshold: liquidation threshold must be greater than base LTV")]
    InvalidThreshold,
    #[msg("Guardian name too long: maximum 32 characters")]
    NameTooLong,
    #[msg("Cooldown already started")]
    CooldownAlreadyStarted,
    #[msg("This instruction is only available on devnet")]
    DevnetOnly,
}
