use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction( candy_machine_id: Pubkey, whitelisted_address: Pubkey, whitelist_type: WLType)]
pub struct CreateWhitelistAccount<'info> {
    #[account(init_if_needed, 
        payer = payer, 
        //space = 8 + std::mem::size_of::<WalletWhitelist>,
        space = 8 + WalletWhitelist::SIZE,
        seeds = [b"wallet-whitelist".as_ref(), whitelist_type.to_string().as_bytes(), whitelisted_address.as_ref(), candy_machine_id.as_ref()], 
        bump
    )]
    pub wallet_whitelist_account: Account<'info, WalletWhitelist>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateWhitelistAccount>, candy_machine_id: Pubkey, whitelisted_address: Pubkey, whitelist_type: WLType, number_of_whitelist_spots: u8 ) -> Result<()>{
    let wallet_whitelist_account = &mut ctx.accounts.wallet_whitelist_account;
    wallet_whitelist_account.candy_machine_id = candy_machine_id;
    wallet_whitelist_account.whitelisted_address = whitelisted_address;
    wallet_whitelist_account.whitelist_type = whitelist_type;
    wallet_whitelist_account.number_of_whitelist_spots = number_of_whitelist_spots;
    wallet_whitelist_account.bump = *ctx.bumps.get("wallet_whitelist_account").unwrap();
    Ok(())
}
