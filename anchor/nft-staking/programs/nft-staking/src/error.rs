use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Max staked amount reached")]
    MaxStakedReached,
    #[msg("Freeze duration not completed")]
    TimeNotElapsed,
    #[msg("Insufficient points to claim rewards")]
    InsufficientPoints,
    #[msg("Stake config does not have vault authority")]
    InvalidVaultAuthority,
}
