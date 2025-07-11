#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("F7HWH1gqNiDXdaTBtwUnRAqAGBkKPWGCaCAwidxBK97c");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, amount, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Make>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take()
    }
}
