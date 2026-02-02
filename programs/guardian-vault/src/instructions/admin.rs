use anchor_lang::prelude::*;
use crate::state::{VaultState, GuardianList, GuardianInfo};
use crate::events::{EmergencyPause, GuardianAdded};
use crate::errors::VaultError;

#[derive(Accounts)]
pub struct AdminConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.bump,
        has_one = authority
    )]
    pub vault_state: Account<'info, VaultState>,
}

#[derive(Accounts)]
pub struct AddGuardian<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault_state"],
        bump = vault_state.bump,
        has_one = authority
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 4 + (10 * (32 + 32 + 2)) + 1 + 1, // Space for Vec<GuardianInfo> + count + bump. Approx.
        seeds = [b"guardian_list"],
        bump
    )]
    pub guardian_list: Account<'info, GuardianList>,

    pub system_program: Program<'info, System>,
}

pub fn emergency_pause(ctx: Context<AdminConfig>) -> Result<()> {
    ctx.accounts.vault_state.paused = true;
    emit!(EmergencyPause {
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

pub fn update_oracle(ctx: Context<AdminConfig>, new_price_feed: Pubkey) -> Result<()> {
    // Basic validation could check if account exists, but for now just update key
    ctx.accounts.vault_state.skr_price_feed = new_price_feed;
    Ok(())
}

pub fn add_guardian(ctx: Context<AddGuardian>, guardian_pubkey: Pubkey, name: String, commission_bps: u16) -> Result<()> {
    let guardian_list = &mut ctx.accounts.guardian_list;
    
    // Validation
    require!(commission_bps <= 700, VaultError::CommissionTooHigh); // Max 7%
    // MEDIUM-04 FIX: Limit name length
    require!(name.len() <= 32, VaultError::NameTooLong);
    
    // Check if exists
    for g in &guardian_list.guardians {
        if g.pubkey == guardian_pubkey {
            return Err(VaultError::GuardianAlreadyWhitelisted.into());
        }
    }

    let info = GuardianInfo {
        pubkey: guardian_pubkey,
        name: name.clone(),
        commission_bps,
    };
    
    guardian_list.guardians.push(info);
    guardian_list.count += 1;
    if guardian_list.guardians.len() == 1 {
         guardian_list.bump = *ctx.bumps.get("guardian_list").unwrap();
    }

    emit!(GuardianAdded {
        guardian_pubkey,
        name,
    });

    Ok(())
}


