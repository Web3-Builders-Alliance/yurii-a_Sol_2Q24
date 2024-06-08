mod contexts;
mod state;

use anchor_lang::prelude::*;
use contexts::*;

declare_id!("5q5k6oS5j3M7MJQg6YHLuJcK7nMzqBeDmJs9bMAerj3Q");

#[program]
pub mod my_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, deposit: u64, receive:u64) -> Result<()> {
        ctx.accounts.init(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}
