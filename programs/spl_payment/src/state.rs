use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub owner: Pubkey, // pubkey of owner
    pub token_mint: Pubkey, // spl_token
    pub vault: Pubkey, // address of SPL token
}

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub address: Pubkey, // wallet address
    pub amount: u64, // amount staked
    pub updated_time: i64, // last updated time
}
