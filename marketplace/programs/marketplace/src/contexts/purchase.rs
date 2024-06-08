use anchor_lang::{prelude::*, system_program};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};
use anchor_spl::token::{close_account, transfer_checked, TransferChecked};
use system_program::Transfer;

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
        close = maker,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump = marketplace.rewards_bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn pay_to_maker(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        system_program::transfer(cpi_ctx, self.listing.price)?;

        Ok(())
    }

    pub fn collect_fee(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let total_fee = calculate_fee(self.listing.price, self.marketplace.fee);

        system_program::transfer(cpi_ctx, total_fee)?;

        Ok(())
    }

    pub fn release_nft(&mut self) -> Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let marketplace_key = self.marketplace.to_account_info().key();
        let maker_mint_key = self.maker_mint.to_account_info().key();
        let seeds = &[
            marketplace_key.as_ref(),
            maker_mint_key.as_ref(),
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        self.listing.close(self.maker.to_account_info())?;

        Ok(())
    }

    pub fn send_rewards(&mut self) -> Result<()> {
        todo!("Send rewards to the taker");
    }
}

fn calculate_fee(amount: u64, fee_basis_points: u16) -> u64 {
    // Basis points to fraction (e.g., 250 basis points is 0.025)
    // 1 basis point = 0.01%
    let fee_fraction = fee_basis_points as u64;

    // Calculate the fee
    // amount * (fee_basis_points / 10_000)
    // is the same as amount * fee_basis_points / 10_000
    let fee = amount.saturating_mul(fee_fraction).checked_div(10_000).unwrap_or(0);

    fee
}

// [X] Send the price from the taker to the maker (SOL)
// [X] Send the fee from the taker to the treasury (SOL)
// [X] Send the NFT from the vault to the taker
// [ ] Optional: Send the rewards to the taker