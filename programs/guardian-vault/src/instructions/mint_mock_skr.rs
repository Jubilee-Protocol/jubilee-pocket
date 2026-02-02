use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo};
use anchor_spl::associated_token::AssociatedToken;
use crate::state::VaultState;

#[derive(Accounts)]
pub struct MintMockSkr<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        mint::authority = vault_state,
    )]
    pub mock_skr_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mock_skr_mint,
        associated_token::authority = user
    )]
    pub user_skr_account: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"vault_state"],
        bump = vault_state.bump
    )]
    pub vault_state: Account<'info, VaultState>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<MintMockSkr>, amount: u64) -> Result<()> {
    // Only allow on devnet/testnet logic could be added here
    // For now, we rely on the fact that the authority is the vault PDA 
    // and this instruction exists.

    let seeds = &[
        b"vault_state".as_ref(),
        &[ctx.accounts.vault_state.bump],
    ];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mock_skr_mint.to_account_info(),
            to: ctx.accounts.user_skr_account.to_account_info(),
            authority: ctx.accounts.vault_state.to_account_info(),
        },
        signer,
    );

    token::mint_to(cpi_ctx, amount)?;
    
    msg!("Minted {} Mock SKR to {}", amount, ctx.accounts.user.key());

    Ok(())
}
