use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};
use crate::state::escrow::*;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub maker: SystemAccount<'info>,
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"escrow",
            escrow.maker.key().as_ref(),
            escrow.seed.to_le_bytes().as_ref(),
        ],
        bump=escrow.bump,
        has_one=mint_a,
        has_one=mint_b,
        close=maker,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        associated_token::mint=mint_a,
        associated_token::authority=taker,
        payer=taker,
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=mint_b,
        associated_token::authority=taker,
    )]
    pub taker_ata_b: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        associated_token::mint=mint_b,
        associated_token::authority=escrow.maker,
        payer=taker,
    )]
    pub maker_ata_b: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
impl<'info> Take<'info> {
    pub fn take(&mut self) -> Result<()>{
        // Transfer from Taker Mint B to Maker Mint B
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.escrow.receive)?;

        Ok(())
    }
    pub fn withdraw_and_close(&mut self) -> Result<()> {
        // Transfer from Vault to Taker Mint A
        let seeds = &[
            b"vault",
            self.maker.key.as_ref(),
            &[self.escrow.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_context, self.vault.amount)?;

        self.escrow.close(self.maker.to_account_info())?;

        Ok(())
    }
}