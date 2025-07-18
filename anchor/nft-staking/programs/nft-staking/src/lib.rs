#![allow(unexpected_cfgs)]
#![allow(deprecated)]

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
// pub use state::*;

declare_id!("to6yPLrYENHE9rRY4jc4DPw4Z3qzTYtwhhFuyKUkgtW");

#[program]
pub mod nft_staking {
    // use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     initialize_config::handler(ctx)
    // }
}
