use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketPlace {
    pub admin: Pubkey,
    pub fee: u16,
    #[max_len(32)]
    pub name: String,
    pub rewards_bump: u8,
    pub treasury_bump: u8,
    pub bump: u8,
}