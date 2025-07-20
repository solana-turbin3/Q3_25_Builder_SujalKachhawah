use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::instructions::{ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts}, MasterEditionAccount, Metadata}, token::{transfer as token_transfer, revoke, Mint, Revoke, Token, TokenAccount, Transfer as TokenTransfer}};

use crate::{Listing, MarketPlace, Treasury};
use crate::error::ErrorCode;

#[derive(Accounts)] 
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub admin: SystemAccount<'info>,
    pub seller: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_mint_ata: Account<'info, TokenAccount>,

    #[account(
        associated_token::mint = mint,
        associated_token::authority = seller,
    )]
    pub seller_mint_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = seller,
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

    #[account(
        seeds = [
            b"marketplace",
            admin.key().as_ref()
        ],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(
        seeds = [
            b"treasury"
        ],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,
    
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Buy<'info> {
    pub fn buy_nft(&self) -> Result<()> {
        let seeds = &[
            b"listing",
            self.seller.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref()
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

        let fee = self.marketplace.fee as u64;
        let fee_numerator = 10_000u64 - fee;
        let fee_denominator = 10_000u64;
    
        let price_in_lamports = LAMPORTS_PER_SOL * self.listing.price as u64;

        let amount = price_in_lamports
                    .checked_mul(fee_numerator)
                    .ok_or(ErrorCode::MathError)?
                    .checked_div(fee_denominator)
                    .ok_or(ErrorCode::MathError)?;
        let fee_paid = price_in_lamports - amount;

        let buy_cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.seller.to_account_info(),
        };

        let fee_cpi_accounts = Transfer {
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let buy_cpi_ctx = CpiContext::new(self.system_program.to_account_info(), buy_cpi_accounts);
        let fee_cpi_ctx = CpiContext::new(self.system_program.to_account_info(), fee_cpi_accounts);

        transfer(buy_cpi_ctx, amount)?;
        transfer(fee_cpi_ctx, fee_paid)?;

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TokenTransfer {
            from: self.seller_mint_ata.to_account_info(),
            to: self.buyer_mint_ata.to_account_info(),
            authority: self.seller.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        token_transfer(cpi_ctx, 1)?;

        let revoke_cpi_program = self.token_program.to_account_info();

        let revoke_accounts = Revoke {
            source: self.seller_mint_ata.to_account_info(),
            authority: self.seller.to_account_info()
        };

        let revoke_cpi_ctx = CpiContext::new_with_signer(revoke_cpi_program, revoke_accounts, signer_seeds);

        revoke(revoke_cpi_ctx)
    }
}