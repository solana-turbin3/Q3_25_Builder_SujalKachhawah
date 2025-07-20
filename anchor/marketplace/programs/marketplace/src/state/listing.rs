use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub seller: Pubkey,
    pub mint: Pubkey,
    pub price: u16,
    pub bump: u8
}