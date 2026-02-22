pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("wy7kkPnizRCbXvrG6fBkuat6q8AwbwTgnjxhZWcg3Si");

#[program]
pub mod guardian_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        harvest_fee_bps: u16,
        base_ltv_bps: u16,
        skr_holder_bonus_bps: u16,
        cooldown_period: i64,
        liquidation_threshold_bps: u16,
        liquidation_penalty_bps: u16,
    ) -> anchor_lang::Result<()> {
        instructions::initialize::handler(
            ctx,
            harvest_fee_bps,
            base_ltv_bps,
            skr_holder_bonus_bps,
            cooldown_period,
            liquidation_threshold_bps,
            liquidation_penalty_bps,
        )
    }

    /// DEV-ONLY: Mints mock SKR tokens for testing. Returns error on mainnet builds.
    #[allow(unused_variables)]
    pub fn mint_mock_skr(ctx: Context<MintMockSkr>, amount: u64) -> anchor_lang::Result<()> {
        #[cfg(not(feature = "devnet"))]
        {
            return Err(crate::errors::VaultError::DevnetOnly.into());
        }
        #[cfg(feature = "devnet")]
        {
            instructions::mint_mock_skr::handler(ctx, amount)
        }
    }

    pub fn deposit_skr_and_borrow(ctx: Context<DepositSkrAndBorrow>, skr_amount: u64) -> anchor_lang::Result<()> {
         instructions::deposit_skr_and_borrow::handler(ctx, skr_amount)
    }

    pub fn init_user_loan(ctx: Context<InitUserLoan>) -> anchor_lang::Result<()> {
        instructions::deposit_skr_and_borrow::init_user_loan(ctx)
    }

    pub fn harvest_repay(ctx: Context<HarvestRepay>) -> anchor_lang::Result<()> {
        instructions::harvest_repay::handler(ctx)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>) -> anchor_lang::Result<()> {
        instructions::withdraw_collateral::handler(ctx)
    }

    pub fn liquidate_loan(ctx: Context<LiquidateLoan>) -> anchor_lang::Result<()> {
        instructions::liquidate_loan::handler(ctx)
    }

    pub fn emergency_pause(ctx: Context<AdminConfig>) -> anchor_lang::Result<()> {
        instructions::admin::emergency_pause(ctx)
    }

    pub fn unpause(ctx: Context<AdminConfig>) -> anchor_lang::Result<()> {
        instructions::admin::unpause(ctx)
    }

    pub fn add_guardian(ctx: Context<AddGuardian>, guardian_pubkey: Pubkey, name: String, commission_bps: u16) -> anchor_lang::Result<()> {
        instructions::admin::add_guardian(ctx, guardian_pubkey, name, commission_bps)
    }

    pub fn update_oracle(ctx: Context<AdminConfig>, new_price_feed: Pubkey) -> anchor_lang::Result<()> {
        instructions::admin::update_oracle(ctx, new_price_feed)
    }


}

