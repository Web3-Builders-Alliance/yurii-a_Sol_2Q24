mod contexts;
mod state;

use anchor_lang::prelude::*;

declare_id!("5q5k6oS5j3M7MJQg6YHLuJcK7nMzqBeDmJs9bMAerj3Q");

#[program]
pub mod my_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
