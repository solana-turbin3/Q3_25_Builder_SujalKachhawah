#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

use crate::{MarketPlace, Treasury};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + MarketPlace::INIT_SPACE,
        seeds = [
            b"marketplace",
        ],
        bump,
    )]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(
        init,
        payer = admin,
        space = 8 + Treasury::INIT_SPACE,
        seeds = [
            b"treasury"
        ],
        bump
    )]
    pub treasury: Account<'info, Treasury>,

    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(MarketPlace { 
            admin: self.admin.to_account_info().key(),
            fee, 
            name,
            rewards_bump: 0,
            treasury_bump: 0, 
            bump: bumps.marketplace
        });

        self.treasury.set_inner(Treasury { 
            admin: self.admin.to_account_info().key(),
            bump: bumps.treasury
        });

        Ok(())
    }
}

