use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata, MetadataAccount}, token::{approve, Approve, Mint, Token, TokenAccount}};

use crate::{Listing};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub collection: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_mint_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        space = 8 + Listing::INIT_SPACE,
        seeds = [
            b"listing",
            seller.key().as_ref(),
            mint.key().as_ref()
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

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

impl<'info> List<'info> {
    pub fn list_nft(&mut self, price: u16, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing { 
            seller: self.seller.to_account_info().key(),
            mint: self.mint.to_account_info().key(),
            price,
            bump: bumps.listing
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            to: self.seller_mint_ata.to_account_info(),
            delegate: self.listing.to_account_info(),
            authority: self.seller.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        let seeds = &[
            b"listing",
            self.seller.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &[self.listing.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        FreezeDelegatedAccountCpi::new(
            &self.metadata_program.to_account_info(), 
            FreezeDelegatedAccountCpiAccounts {
                delegate: &self.listing.to_account_info(),
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_account: &self.seller_mint_ata.to_account_info(),
                token_program: &self.token_program.to_account_info()
            }
        ).invoke_signed(signer_seeds)?;

        Ok(())
    }
}