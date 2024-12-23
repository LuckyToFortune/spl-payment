mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("ADJTuM8BMjCupygz4jWP8GPhh3p4cfveYs5h7mfCDw1Y");

#[program]
pub mod spl_payment {
    use super::*;

    // owner function
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn update_owner(ctx: Context<SetData>, new_owner: Pubkey) -> Result<()> {
        instructions::update_owner(ctx, new_owner)
    }

    // user function
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw(ctx, amount)
    }
}
