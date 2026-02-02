use anchor_lang::prelude::*;

#[account]
pub struct VaultState {
    pub authority: Pubkey,
    pub labs_treasury: Pubkey,
    pub total_collateral: u64,
    pub total_debt: u64,
    pub harvest_fee_bps: u16,
    pub base_ltv_bps: u16,
    pub skr_holder_bonus_bps: u16,
    pub cooldown_period: i64,
    pub liquidation_threshold_bps: u16,
    pub liquidation_penalty_bps: u16,
    pub paused: bool,
    pub skr_price_feed: Pubkey,
    pub bump: u8,
}

#[account]
pub struct UserLoan {
    pub owner: Pubkey,
    pub collateral_amount: u64,
    pub debt_amount: u64,
    pub created_at: i64,
    pub last_harvest: i64,
    pub unstake_requested_at: i64,
    pub guardian_pubkey: Pubkey,
    pub initial_ltv_bps: u16,
    pub bump: u8,
}

#[account]
pub struct GuardianList {
    pub guardians: Vec<GuardianInfo>,
    pub count: u8,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct GuardianInfo {
    pub pubkey: Pubkey,
    pub name: String,
    pub commission_bps: u16,
}
