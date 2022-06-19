use instructions::*;

pub mod error;
pub mod instructions;
use state::wallet_whitelist::WLType;
pub mod state;

use anchor_lang::prelude::*;

declare_id!("2EKXS53suVyMEn4nsRGSSY167vJ9PGeLR1b7dwzamqkX");

#[program]
pub mod wallet_whitelisting_for_minting {
    use super::*;

    pub fn create_whitelist_account(
        ctx: Context<CreateWhitelistAccount>,
        candy_machine_id: Pubkey,
        whitelisted_address: Pubkey,
        whitelist_type: WLType,
        number_of_whitelist_spots: u8,
    ) -> Result<()> {
        instructions::create_whitelist_account::handler(
            ctx,
            candy_machine_id,
            whitelisted_address,
            whitelist_type,
            number_of_whitelist_spots,
        )
    }

    pub fn decrease_whitelist_count(ctx: Context<DecreaseWhitelistSpots>, count: u8) -> Result<()> {
        instructions::decrease_whitelist_count::handler(ctx, count)
    }
}
