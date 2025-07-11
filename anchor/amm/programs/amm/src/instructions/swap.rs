#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        transfer, Mint, TokenAccount, Transfer
    },
};
use constant_product_curve::ConstantProduct;

use crate::{state::Config, error::AmmError};
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x : Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y : Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Account<'info, TokenAccount>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(&self, is_x: bool, amount: u64, min_amount_out: u64) -> Result<()> {
        require!(amount != 0, AmmError::InvalidAmount);

        self.transfer_user_to_vault(is_x, amount)?;

        let fee_bps = self.config.fee as u64;
        let fee_numerator = 10_000u64 - fee_bps;
        let fee_denominator = 10_000u64;

        let adjusted_amount_in  = amount
            .checked_mul(fee_numerator)
            .ok_or(AmmError::CurveError)?
            .checked_div(fee_denominator)
            .ok_or(AmmError::CurveError)?;

        let (reserve_in, reserve_out) = if is_x {
            (self.vault_x.amount, self.vault_y.amount)
        } else {
            (self.vault_y.amount, self.vault_x.amount)
        };

        let amount_out = ConstantProduct::delta_y_from_x_swap_amount(
            reserve_in, 
            reserve_out, 
            adjusted_amount_in).map_err(|_| AmmError::CurveError)?;

        require!(amount_out >= min_amount_out, AmmError::SlippageExceeded);

        self.transfer_vault_to_user(!is_x, amount_out)
    }

    pub fn transfer_user_to_vault(&self, is_x: bool, amount: u64) -> Result<()> {
        let vault: &Account<'info, TokenAccount>;
        let token: &Account<'info, TokenAccount>;

        match is_x {
            true => {
                vault = &self.vault_x;
                token = &self.user_x;
            },
            false => {
                vault = &self.vault_y;
                token = &self.user_y;
            }
        }

        require!(token.amount >= amount, AmmError::InsufficientBalance);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: token.to_account_info(),
            to: vault.to_account_info(),
            authority: self.user.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(ctx, amount)
    }

    pub fn transfer_vault_to_user(&self, is_x:bool, amount: u64) -> Result<()> {
        let vault: &Account<'info, TokenAccount>;
        let token: &Account<'info, TokenAccount>;

        match is_x {
            true => {
                vault = &self.vault_x;
                token = &self.user_x;
            },
            false => {
                vault = &self.vault_y;
                token = &self.user_y;
            }
        }

        require!(vault.amount >= amount, AmmError::InsufficientBalance);

        let seeds = &[
            b"config",
            &self.config.seed.to_le_bytes()[..],
            &[self.config.config_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: vault.to_account_info(),
            to: token.to_account_info(),
            authority: self.config.to_account_info()
        };

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(ctx, amount)
    }
}