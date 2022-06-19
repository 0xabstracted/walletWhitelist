use instructions::*;
use state::*;

pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("2EKXS53suVyMEn4nsRGSSY167vJ9PGeLR1b7dwzamqkX");

#[program]
pub mod wallet_whitelisting_for_minting {
    use super::*;

    pub fn create_whitelist_account(
        ctx: Context<CreateWhitelistAccount>,
        data: WalletWhitelistData,
    ) -> Result<()> {
        instructions::create_whitelist_account::handler(ctx, data)
    }

    pub fn decrease_whitelist_count(ctx: Context<DecreaseWhitelistSpots>, count: u8) -> Result<()> {
        instructions::decrease_whitelist_count::handler(ctx, count)
    }
}
