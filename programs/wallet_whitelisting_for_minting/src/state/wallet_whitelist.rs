use std::fmt::Debug;

use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub enum WLType {
    FOUR,
    THREE,
    TWO,
    ONE,
    NULL,
}

impl Default for WLType {
    fn default() -> Self {
        WLType::NULL
    }
}

#[account]
#[derive(Default, Debug)]
pub struct WalletWhitelist {
    pub candy_machine_id: Pubkey,
    pub whitelisted_address: Pubkey,
    pub whitelist_type: WLType,
    pub number_of_whitelist_spots: u8,
    pub bump: u8,
    _reserved: [u8; 2],
}

impl WalletWhitelist {
    pub const SIZE: usize = 32 + 32 + 4 + 1 + 1 + 2;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default, Debug)]
pub struct WalletWhitelistData {
    pub candy_machine_id: Pubkey,
    pub whitelisted_address: Pubkey,
    pub whitelist_type: WLType,
    pub number_of_whitelist_spots: u8,
}
