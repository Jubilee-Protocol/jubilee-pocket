use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use crate::state::{VaultState, UserLoan};
use crate::events::RewardHarvested;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct HarvestRepay<'info> {
    // CRITICAL-05 FIX: Require caller to be the loan owner
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_loan", caller.key().as_ref()],
        bump = user_loan.bump,
        constraint = user_loan.owner == caller.key() @ VaultError::Unauthorized
    )]
    pub user_loan: Account<'info, UserLoan>,

    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.bump
    )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: Treasury to receive harvest fees (SKR)
    #[account(mut, address = vault_state.labs_treasury)]
    pub labs_treasury_skr: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mock_skr_mint,
        associated_token::authority = vault_state
    )]
    pub vault_skr_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = jusdi_mint,
        associated_token::authority = vault_state
    )]
    pub vault_jusdi_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub jusdi_mint: Box<Account<'info, Mint>>,
    
    pub mock_skr_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<HarvestRepay>) -> Result<()> {
    // CRITICAL-02 FIX: Check pause state
    let vault_state = &mut ctx.accounts.vault_state;
    require!(!vault_state.paused, VaultError::Paused);

    // 1. Calculate Rewards
    let user_loan = &mut ctx.accounts.user_loan;
    let current_timestamp = Clock::get()?.unix_timestamp;
    
    let time_elapsed = current_timestamp.checked_sub(user_loan.last_harvest).unwrap_or(0);
    if time_elapsed <= 0 {
        return Ok(());
    }

    // Devnet Simulation: 7% APY
    // rewards = collateral * 0.07 * (time_elapsed / 31536000)
    // Calculation: collateral * 7 * time / (100 * 31536000)
    
    let _seconds_per_year = 31536000u128; // 365 * 24 * 3600
    let collateral_u128 = user_loan.collateral_amount as u128;
    let time_u128 = time_elapsed as u128;
    
    // rewards = collateral * 7 * time / 100 / seconds_per_year
    // rewards = (collateral * 7 * time) / (3153600000)
    
    // HIGH-02 FIX: Replace unwrap() with ok_or()
    let rewards_earned_u128 = collateral_u128
        .checked_mul(7).ok_or(VaultError::MathOverflow)?
        .checked_mul(time_u128).ok_or(VaultError::MathOverflow)?
        .checked_div(3153600000).ok_or(VaultError::MathOverflow)?;
        
    let rewards_earned = rewards_earned_u128 as u64;
    
    if rewards_earned == 0 {
        user_loan.last_harvest = current_timestamp;
        return Ok(());
    }

    // 2. Take Fee
    let fee_bps = vault_state.harvest_fee_bps as u64;
    let fee_amount = rewards_earned.checked_mul(fee_bps).ok_or(VaultError::MathOverflow)?.checked_div(10000).ok_or(VaultError::MathOverflow)?;
    let net_rewards = rewards_earned.checked_sub(fee_amount).ok_or(VaultError::MathOverflow)?;

    // 3. "Swap" SKR to jUSDi/USDC to repay debt
    // On Devnet, we assume 1 Mock SKR = $X. 
    // And burn jUSDi debt equivalent to net_rewards value?
    // Wait, the rewards are in SKR (staking rewards).
    // We need to convert SKR rewards to jUSDi to burn debt.
    // Or we assume the vault "Sells" SKR rewards for jUSDi (which it burns).
    // Logic:
    // Vault Mint newly minted SKR rewards? No, staking increases balance.
    // In LST, the balance increases or exchange rate changes. 
    // Here we simulate "rewards_earned" as new tokens? 
    // Let's assume we mint Mock SKR to vault to simulate rewards accumulation?
    // Or just assume vault has SKR buffer?
    // Simpler: Just reduce debt based on calculated value.
    // Assume 1 SKR = $10 (fixed for simulation or fetch oracle).
    
    // Fetch oracle? Not passed in this instruction.
    // Let's assume passed validation earlier or use cached Pyth?
    // For Devnet Hackathon, purely simulated:
    // Assume 1 SKR = 10 jUSDi.
    // debt_reduction = net_rewards * 10.
    
    // Better: Require oracle account to precise calculation.
    // For now, let's use a fixed rate of $10 for harvest simulation to save complexity, 
    // or add Pyth account to context.
    // I'll skip oracle for harvest_repay in this pass to keep it simple, or user can update.
    // MEDIUM-05: Devnet simulation — $10/SKR. TODO(MAINNET): Replace with Pyth oracle lookup.
    let simulated_price = 10u64;
    let debt_reduction = net_rewards.checked_mul(simulated_price).ok_or(VaultError::MathOverflow)?;
    
    // Burn jUSDi from vault (assuming vault has some or we just reduce the user debt number?)
    // Real flow: Swap SKR -> USDC, Buy jUSDi -> Burn.
    // Here: Update UserLoan debt.
    // But we need to account for the "fee" transfer.
    
    // Transfer fee (SKR) from Vault to Treasury?
    // Assuming the vault has these SKR tokens (rewards).
    // Implementation details: "Simulate ... rewards".
    // We will Mint new Mock SKR to vault to represent rewards? 
    // No, logic says "Transfer fee to labs_treasury_skr".
    // This implies Vault has these tokens.
    // But we haven't increased Vault's SKR balance.
    // So "harvest" in Mock scenario implies Minting new SKR?
    // `mint_mock_skr` mints to USER.
    // `harvest` should probably Mint rewards to Vault first?
    // Or just skip token transfers and only update accounting for hackathon?
    // "Transfer fee to treasury" is explicit spec.
    // So I need to Mint `fee_amount` SKR to Treasury?
    // And Mint `net_rewards` SKR to Vault (which is then swapped/burned).
    
    // Simplified Hackathon Logic:
    // 1. Mint `fee_amount` Mock SKR to Treasury (Simulate fee payout).
    // 2. Reduce user debt by `debt_reduction`.
    // 3. Emit event.
    
    // We accept `mock_skr_mint` in context.
    // We can Mint to Treasury.
    
    let seeds = &[
        b"vault_state".as_ref(),
        &[vault_state.bump],
    ];
    let signer = &[&seeds[..]];

    // Mint Fee to Treasury
    if fee_amount > 0 {
         let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mock_skr_mint.to_account_info(),
                to: ctx.accounts.labs_treasury_skr.to_account_info(),
                authority: vault_state.to_account_info(),
            },
            signer,
        );
        token::mint_to(cpi_ctx, fee_amount)?;
    }
    
    // Reduce Debt
    let new_debt = user_loan.debt_amount.checked_sub(debt_reduction).unwrap_or(0);
    let start_debt = user_loan.debt_amount;
    user_loan.debt_amount = new_debt;
    user_loan.last_harvest = current_timestamp;

    // Update Global Debt
    let actual_reduction = start_debt.checked_sub(new_debt).unwrap();
    vault_state.total_debt = vault_state.total_debt.checked_sub(actual_reduction).unwrap_or(0);
    
    emit!(RewardHarvested {
        user: user_loan.owner,
        rewards_earned,
        fee_taken: fee_amount,
        debt_reduced: actual_reduction,
    });

    Ok(())
}
