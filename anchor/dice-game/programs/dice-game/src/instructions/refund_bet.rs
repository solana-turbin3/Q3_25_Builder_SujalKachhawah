use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::{Bet};

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&self) -> Result<()> {
        let seeds = &[
            b"vault",
            &self.house.key().to_bytes()[..],
            &[self.bet.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer {
                from: self.player.to_account_info(),
                to: self.vault.to_account_info(),
            },
            signer_seeds
        );

        transfer(ctx, self.bet.amount)
    }
}