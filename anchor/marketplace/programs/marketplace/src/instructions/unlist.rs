use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata}, token::{revoke, Mint, Revoke, Token, TokenAccount}};

use crate::{Listing};

#[derive(Accounts)]
pub struct Unlist<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [
            b"listing",
            seller.key().as_ref(),
            mint.key().as_ref()
        ],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>
}

impl<'info> Unlist<'info> {
    pub fn list_nft(&self) -> Result<()> {
        let seeds = &[
            b"listing",
            self.seller.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &[self.listing.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        ThawDelegatedAccountCpi::new(
            &self.metadata_program.to_account_info(), 
            ThawDelegatedAccountCpiAccounts {
                delegate: &self.listing.to_account_info(),
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_account: &self.seller_mint_ata.to_account_info(),
                token_program: &self.token_program.to_account_info()
            }
        ).invoke_signed(signer_seeds)?;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Revoke {
            source: self.seller_mint_ata.to_account_info(),
            authority: self.seller.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        revoke(cpi_ctx)
    }
}