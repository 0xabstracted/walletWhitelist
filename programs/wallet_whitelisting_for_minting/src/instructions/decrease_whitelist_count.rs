use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DecreaseWhitelistSpots<'info> {
    #[account(mut,
        has_one = whitelisted_address,
        has_one = candy_machine_id,
        // seeds = [b"wallet-whitelist", wallet_whitelist_account.whitelist_type.to_string().as_bytes() , whitelisted_address.key().as_ref(), candy_machine_id.key().as_ref()], 
        // bump = wallet_whitelist_account.bump
    )]
    pub wallet_whitelist_account: Account<'info, WalletWhitelist>,
    /// CHECK:
    #[account(constraint = wallet_whitelist_account.candy_machine_id == candy_machine_id.key())]
    pub candy_machine_id: AccountInfo<'info>,
    /// CHECK:
    #[account(constraint = wallet_whitelist_account.whitelisted_address == whitelisted_address.key())]
    // pub whitelisted_address: AccountInfo<'info>,
    pub whitelisted_address: Signer<'info>,
}

pub fn handler(ctx: Context<DecreaseWhitelistSpots>, count: u8) -> Result<()> {
    let wallet_whitelist_account = &mut ctx.accounts.wallet_whitelist_account;
    if count > wallet_whitelist_account.number_of_whitelist_spots {
        return Err(ErrorCode::InvalidNumberofWL.into());
    }
    wallet_whitelist_account.number_of_whitelist_spots -= count;
    Ok(())
}
