use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + std::mem::size_of::<VaultState>(),
        seeds = [b"vault_state"],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: The treasury address where harvest fees are sent
    pub labs_treasury: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    harvest_fee_bps: u16,
    base_ltv_bps: u16,
    skr_holder_bonus_bps: u16,
    cooldown_period: i64,
    liquidation_threshold_bps: u16,
    liquidation_penalty_bps: u16,
) -> Result<()> {
    // HIGH-01 FIX: Validate parameters
    require!(base_ltv_bps <= 8000, VaultError::LTVTooHigh); // Max 80% base LTV
    require!(base_ltv_bps + skr_holder_bonus_bps <= 9000, VaultError::LTVTooHigh); // Max 90% with bonus
    require!(liquidation_threshold_bps > base_ltv_bps + skr_holder_bonus_bps, VaultError::InvalidThreshold);
    require!(liquidation_threshold_bps <= 9500, VaultError::InvalidThreshold); // Max 95%
    require!(harvest_fee_bps <= 2000, VaultError::CommissionTooHigh); // Max 20% fee
    require!(liquidation_penalty_bps <= 1500, VaultError::CommissionTooHigh); // Max 15% penalty
    require!(cooldown_period >= 0, VaultError::GenericError); // Non-negative cooldown

    let vault_state = &mut ctx.accounts.vault_state;
    vault_state.authority = ctx.accounts.authority.key();
    vault_state.labs_treasury = ctx.accounts.labs_treasury.key();
    vault_state.harvest_fee_bps = harvest_fee_bps;
    vault_state.base_ltv_bps = base_ltv_bps;
    vault_state.skr_holder_bonus_bps = skr_holder_bonus_bps;
    vault_state.cooldown_period = cooldown_period;
    vault_state.liquidation_threshold_bps = liquidation_threshold_bps;
    vault_state.liquidation_penalty_bps = liquidation_penalty_bps;
    vault_state.paused = false;
    vault_state.bump = *ctx.bumps.get("vault_state").unwrap();
    
    // Default values for counters
    vault_state.total_collateral = 0;
    vault_state.total_debt = 0;

    Ok(())
}
