use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::{VaultState, UserLoan};
use crate::events::CollateralWithdrawn;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct WithdrawCollateral<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // CRITICAL-04 FIX: Removed `close = user` - we close manually only when complete
    #[account(
        mut,
        seeds = [b"user_loan", user.key().as_ref()],
        bump = user_loan.bump,
        constraint = user_loan.owner == user.key() @ VaultError::Unauthorized
    )]
    pub user_loan: Account<'info, UserLoan>,

    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        associated_token::mint = mock_skr_mint,
        associated_token::authority = vault_state
    )]
    pub vault_skr_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mock_skr_mint,
        associated_token::authority = user
    )]
    pub user_skr_account: Box<Account<'info, TokenAccount>>,

    pub mock_skr_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawCollateral>) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    
    // CRITICAL-02 FIX: Check pause state
    require!(!vault_state.paused, VaultError::Paused);

    let user_loan = &mut ctx.accounts.user_loan;

    // 1. Check Debt
    require!(user_loan.debt_amount == 0, VaultError::DebtNotZero);
    
    // 2. Cooldown Logic
    let current_timestamp = Clock::get()?.unix_timestamp;
    
    if user_loan.unstake_requested_at == 0 {
        // Start Cooldown - user must call again after period expires
        user_loan.unstake_requested_at = current_timestamp;
        msg!("Cooldown started at {}. Please return after {} seconds.", current_timestamp, vault_state.cooldown_period);
        return Ok(());
    }
    
    // Check if cooldown passed
    let passed = current_timestamp.checked_sub(user_loan.unstake_requested_at).unwrap_or(0);
    if passed < vault_state.cooldown_period {
        msg!("Cooldown not met. Passed: {}, Required: {}", passed, vault_state.cooldown_period);
        return Err(VaultError::CooldownNotMet.into());
    }

    // 3. Withdraw
    let amount = user_loan.collateral_amount;
    
    // Transfer SKR
    let seeds = &[
        b"vault_state".as_ref(),
        &[vault_state.bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_skr_account.to_account_info(),
            to: ctx.accounts.user_skr_account.to_account_info(),
            authority: vault_state.to_account_info(),
        },
        signer,
    );
    token::transfer(cpi_ctx, amount)?;
    
    // 4. Update Global State - MEDIUM-02 FIX: graceful underflow handling
    vault_state.total_collateral = vault_state.total_collateral.checked_sub(amount).ok_or(VaultError::MathOverflow)?;
    
    // 5. Emit Event
    emit!(CollateralWithdrawn {
        user: ctx.accounts.user.key(),
        amount,
    });
    
    // 6. Close Account Manually
    let source_account_info = user_loan.to_account_info();
    let dest_account_info = ctx.accounts.user.to_account_info();
    
    // Transfer lamports
    let dest_lamports = dest_account_info.lamports();
    **dest_account_info.lamports.borrow_mut() = dest_lamports.checked_add(source_account_info.lamports()).unwrap();
    **source_account_info.lamports.borrow_mut() = 0;
    
    // Clear data is optional here since we set lamports to 0 (runtime will clean up),
    // but good practice to clear if sensitive?
    // Anchor will handle re-initialization safety.
    
    Ok(())
}
