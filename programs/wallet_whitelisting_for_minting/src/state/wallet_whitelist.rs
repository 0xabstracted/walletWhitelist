use anchor_lang::prelude::*;
use std::fmt;
use std::fmt::Debug;

#[proc_macros::assert_size(4)]
#[repr(C)]
#[derive(Debug, Clone, Copy, AnchorDeserialize, AnchorSerialize)]
pub enum WLType {
    Four,
    Three,
    Two,
    One,
    Null,
}
// pub enum WLType {
//     Four(String),
//     Three(String),
//     Two(String),
//     One(String),
//     Null(String),
// }

impl fmt::Display for WLType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Default for WLType {
    fn default() -> Self {
        WLType::Null
    }
}

#[proc_macros::assert_size(72)]
#[repr(C)]
#[account]
#[derive(Default, Debug)]
pub struct WalletWhitelist {
    pub candy_machine_id: Pubkey,      //32
    pub whitelisted_address: Pubkey,   //32
    pub whitelist_type: WLType,        //4
    pub number_of_whitelist_spots: u8, //1
    pub bump: u8,                      //1
    _reserved: [u8; 2],                //2
}

impl WalletWhitelist {
    pub const SIZE: usize = 32 + 32 + 4 + 1 + 1 + 2;
}
