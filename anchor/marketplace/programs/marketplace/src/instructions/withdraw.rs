#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};

use crate::error::ErrorCode;
use crate::{Treasury};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [
            b"treasury"
        ],
        bump = treasury.bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info> {
    pub fn withdraw_funds(&self, amount: u64) -> Result<()> {
        let amount_in_lamports = amount * LAMPORTS_PER_SOL;
        require!(self.treasury.to_account_info().lamports() >= amount_in_lamports, ErrorCode::InsufficientBalance);

        let seeds = &[
            b"treasury".as_ref()
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.admin.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount_in_lamports)
    }
}

