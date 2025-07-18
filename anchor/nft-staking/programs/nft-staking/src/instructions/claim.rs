use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};

use crate::state::{StakeConfig, UserAccount};
use crate::error::StakeError;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub reward_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub user_reward_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            b"user",
            user.key().as_ref()
        ],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        token::mint = reward_mint,
        token::authority = config
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>
}

impl<'info> Claim<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        require!(self.user_account.points > 0, StakeError::InsufficientPoints);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.reward_vault.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let seeds = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        let rewards = self.user_account.points as u64;
        self.user_account.points = 0;

        transfer(cpi_ctx, rewards)
    }
}