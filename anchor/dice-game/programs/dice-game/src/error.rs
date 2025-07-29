use anchor_lang::prelude::*;

#[error_code]
pub enum DiceError {
    #[msg("")]
    ED25519Program,
    #[msg("")]
    ED25519Accounts,
    #[msg("")]
    ED25519DataLength,
    #[msg("")]
    ED25519Header,
    #[msg("")]
    ED25519Pubkey,
    #[msg("")]
    ED25519Signature,
    #[msg("")]
    OverFlow,
}
