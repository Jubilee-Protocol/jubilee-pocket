use anchor_lang::prelude::*;

#[event]
pub struct LoanCreated {
    pub user: Pubkey,
    pub collateral_amount: u64,
    pub debt_amount: u64,
    pub ltv_bps: u16,
    pub guardian_pubkey: Pubkey,
}

#[event]
pub struct RewardHarvested {
    pub user: Pubkey,
    pub rewards_earned: u64,
    pub fee_taken: u64,
    pub debt_reduced: u64,
}

#[event]
pub struct CollateralWithdrawn {
    pub user: Pubkey,
    pub amount: u64,
}

#[event]
pub struct LoanLiquidated {
    pub user: Pubkey,
    pub liquidator: Pubkey,
    pub collateral_seized: u64,
    pub debt_repaid: u64,
}

#[event]
pub struct EmergencyPause {
    pub timestamp: i64,
}

#[event]
pub struct GuardianAdded {
    pub guardian_pubkey: Pubkey,
    pub name: String,
}
