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

declare_id!("3Vm6Fzxkq1u754HvY7Q7Tk57tDLeGd5kiuCCrNewx6rV");

#[program]
pub mod marketplace {
    // use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // initialize::handler(ctx)
    // }
}
