## error.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided exchange rate is invalid.")]
    InvalidExchangeRate,

    #[msg("Failed to calculate the ROMA amount.")]
    CalculationError,

    #[msg("Token transfer failed.")]
    TransferError,

    #[msg("Operation not allowed in current state.")]
    OperationNotAllowed,

    #[msg("Insufficient funds for the operation.")]
    InsufficientFunds,
}
