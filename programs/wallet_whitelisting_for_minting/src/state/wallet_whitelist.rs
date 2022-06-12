use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct WalletWhitelist {
    pub candy_machine_id: Pubkey,
    pub whitelisted_address: Pubkey,
    pub number_of_whitelist_spots: u8,
    pub bump: u8,
}

impl WalletWhitelist {
    pub const SIZE: usize = 32 + 32 + 1 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct WalletWhitelistData {
    pub candy_machine_id: Pubkey,
    pub whitelisted_address: Pubkey,
    pub number_of_whitelist_spots: u8,
}
