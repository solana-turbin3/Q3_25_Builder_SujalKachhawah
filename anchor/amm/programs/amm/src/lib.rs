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

declare_id!("HvzUsErkRzaYD69nHX85vV32MM357k37yftd9Q6wyDak");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed:u64, fee:u16, ) -> Result<()> {
        ctx.accounts.init(seed, fee, Some(*ctx.accounts.initializer.key), ctx.bumps)
    }
}
