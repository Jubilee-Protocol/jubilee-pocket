use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};
use anchor_spl::associated_token::AssociatedToken;
#[cfg(not(feature = "devnet"))]
use pyth_sdk_solana::state::SolanaPriceAccount;
use crate::state::{VaultState, UserLoan};
use crate::events::LoanCreated;
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct DepositSkrAndBorrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_loan", user.key().as_ref()],
        bump = user_loan.bump
    )]
    pub user_loan: Box<Account<'info, UserLoan>>,

    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.bump
    )]
    pub vault_state: Box<Account<'info, VaultState>>,

    #[account(
        mut,
        associated_token::mint = jusdi_mint,
        associated_token::authority = vault_state
    )]
    pub vault_jusdi_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut, 
        associated_token::mint = mock_skr_mint,
        associated_token::authority = vault_state
    )]
    pub vault_skr_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = jusdi_mint,
        associated_token::authority = user
    )]
    pub user_jusdi_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mock_skr_mint,
        associated_token::authority = user
    )]
    pub user_skr_account: Box<Account<'info, TokenAccount>>,
    
    // The mint of the collateral (mock SKR)
    pub mock_skr_mint: Box<Account<'info, Mint>>,

    // CRITICAL-03 FIX: Validate vault is mint authority
    #[account(
        mut,
        constraint = jusdi_mint.mint_authority.unwrap() == vault_state.key() @ VaultError::InvalidMintAuthority
    )]
    pub jusdi_mint: Box<Account<'info, Mint>>,

    /// CHECK: Pyth price feed account - validated in handler
    #[account(constraint = skr_price_feed.key() == vault_state.skr_price_feed @ VaultError::InvalidPriceFeed)]
    pub skr_price_feed: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DepositSkrAndBorrow>, skr_amount: u64) -> Result<()> {
    let vault_state = &ctx.accounts.vault_state;
    require!(!vault_state.paused, VaultError::Paused);
    
    let current_timestamp = Clock::get()?.unix_timestamp;
    
    // Calculate LTV
    let user_balance_before = ctx.accounts.user_skr_account.amount;
    // 100 SKR = 100 * 10^6
    let bonus_threshold = 100u64.checked_mul(10u64.pow(6)).ok_or(VaultError::MathOverflow)?;
    
    let ltv_bps = if user_balance_before >= bonus_threshold {
        vault_state.base_ltv_bps + vault_state.skr_holder_bonus_bps
    } else {
        vault_state.base_ltv_bps
    };
    
    // PYTH ORACLE INTEGRATION (with devnet fallback)
    #[cfg(feature = "devnet")]
    let (price_i64, expo) = (10_000_000i64, -6i32); // $10.00 fallback for devnet
    
    #[cfg(not(feature = "devnet"))]
    let (price_i64, expo) = {
        let price_feed = SolanaPriceAccount::account_info_to_feed(&ctx.accounts.skr_price_feed.to_account_info())
            .map_err(|_| VaultError::InvalidPriceFeed)?;
        let price_data = price_feed.get_price_no_older_than(current_timestamp, 60)
            .ok_or(VaultError::StaleOracle)?;
        (price_data.price, price_data.expo)
    };
    
    require!(price_i64 > 0, VaultError::InvalidPriceFeed);
    let price_u64 = price_i64 as u64;
    
    let _decimals = 6; // Mock SKR and jUSDi
    
    // Calculation: collateral_value = (amount * price) * 10^(decimals + expo - decimals) = amount * price * 10^expo
    // If expo is -8, we divide by 10^8.
    
    // Using u128 for intermediate calc
    let amount_u128 = skr_amount as u128;
    let price_u128 = price_u64 as u128;
    let value_u128 = amount_u128.checked_mul(price_u128).ok_or(VaultError::MathOverflow)?;
    
    let scale_adj = expo.abs() as u32;
    // value (raw) needs to be shifted by expo
    // if expo (-8) -> value is X * 10^-8 USD.
    // We want jUSDi amount (6 decimals).
    // So output should be USD value * 10^6.
    
    // target_value = value * 10^6 * 10^expo
    
    let borrow_amount_full_ltv = if expo < 0 {
        // value_u128 / 10^abs(expo)
        // This assumes source_decimals == target_decimals (6).
        // If they differed, we'd adjust by 10^(target - source).
        let den = 10u128.pow(scale_adj);
        value_u128.checked_div(den).ok_or(VaultError::MathOverflow)?
    } else {
        // value_u128 * 10^expo
        let factor = 10u128.pow(expo as u32);
        value_u128.checked_mul(factor).ok_or(VaultError::MathOverflow)?
    };
    
    // Apply LTV
    // amount = full_val * ltv_bps / 10000
    let borrow_amount = borrow_amount_full_ltv
        .checked_mul(ltv_bps as u128).ok_or(VaultError::MathOverflow)?
        .checked_div(10000u128).ok_or(VaultError::MathOverflow)?;
        
    let borrow_amount_u64 = borrow_amount as u64;

    // 4. Transfer SKR
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_skr_account.to_account_info(),
        to: ctx.accounts.vault_skr_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, skr_amount)?;

    // 5. Mint jUSDi
    let seeds = &[
        b"vault_state".as_ref(),
        &[vault_state.bump],
    ];
    let signer = &[&seeds[..]];

    let mint_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.jusdi_mint.to_account_info(),
            to: ctx.accounts.user_jusdi_account.to_account_info(),
            authority: vault_state.to_account_info(),
        },
        signer,
    );
    token::mint_to(mint_ctx, borrow_amount_u64)?;

    // 6. Update Loan State
    let user_loan = &mut ctx.accounts.user_loan;
    // user_loan.owner set in init
    user_loan.collateral_amount = user_loan.collateral_amount.checked_add(skr_amount).ok_or(VaultError::MathOverflow)?;
    user_loan.debt_amount = user_loan.debt_amount.checked_add(borrow_amount_u64).ok_or(VaultError::MathOverflow)?;
    user_loan.last_harvest = current_timestamp;
    
    // Set LTV snapshot if this is fresh
    if user_loan.collateral_amount == skr_amount {
         user_loan.initial_ltv_bps = ltv_bps;
    }
    
    // Update Global Stats
    let vault_state_mut = &mut ctx.accounts.vault_state;
    vault_state_mut.total_collateral = vault_state_mut.total_collateral.checked_add(skr_amount).ok_or(VaultError::MathOverflow)?;
    vault_state_mut.total_debt = vault_state_mut.total_debt.checked_add(borrow_amount_u64).ok_or(VaultError::MathOverflow)?;

    emit!(LoanCreated {
        user: ctx.accounts.user.key(),
        collateral_amount: skr_amount,
        debt_amount: borrow_amount_u64,
        ltv_bps,
        guardian_pubkey: Pubkey::default(), // TODO: Add Guardian selection logic
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitUserLoan<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, // Always init, not if_needed. One loan per user.
        payer = user,
        space = 8 + std::mem::size_of::<UserLoan>(),
        seeds = [b"user_loan", user.key().as_ref()],
        bump
    )]
    pub user_loan: Account<'info, UserLoan>,

    pub system_program: Program<'info, System>,
}

pub fn init_user_loan(ctx: Context<InitUserLoan>) -> Result<()> {
    let user_loan = &mut ctx.accounts.user_loan;
    user_loan.owner = ctx.accounts.user.key();
    user_loan.created_at = Clock::get()?.unix_timestamp;
    user_loan.bump = *ctx.bumps.get("user_loan").unwrap();
    user_loan.debt_amount = 0;
    user_loan.collateral_amount = 0;
    Ok(())
}
