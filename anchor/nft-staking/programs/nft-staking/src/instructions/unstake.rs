use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{mpl_token_metadata::instructions::{
        ThawDelegatedAccountCpi,
        ThawDelegatedAccountCpiAccounts,
    },MasterEditionAccount, Metadata }, token::{
        Mint, Revoke, Token, TokenAccount, revoke
    }
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};
use crate::error::StakeError;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
            ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        close = user,
        seeds = [
            b"stake",
            mint.key().as_ref(),
            config.key().as_ref()
        ],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds = [
            b"user",
            user.key().as_ref()
        ],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.staked_at) / 86400) as u32;
        require!(time_elapsed >= self.config.freeze_period, StakeError::TimeNotElapsed);

        self.user_account.points += (self.config.points_per_stake as u32) * time_elapsed;

        let token_program = &self.token_program.to_account_info();
        let delegate = &self.stake_account.to_account_info();
        let mint = &self.mint.to_account_info();
        let edition = &self.edition.to_account_info();
        let token_account = &self.user_mint_ata.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        let seeds = &[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        ThawDelegatedAccountCpi::new(
            metadata_program, 
            ThawDelegatedAccountCpiAccounts { 
                delegate,
                token_account, 
                edition,
                mint, 
                token_program, 
            }
        ).invoke_signed(signer_seeds)?;

        self.user_account.amount_staked -= 1;
        
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Revoke {
            authority: self.user.to_account_info(),
            source: self.user_mint_ata.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        revoke(cpi_context)
    }
}