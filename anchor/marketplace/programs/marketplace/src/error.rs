use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Math Error")]
    MathError,
    #[msg("Insufficient Balance")]
    InsufficientBalance,
}
