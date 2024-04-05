use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized access attempt.")]
    UnauthorizedAccess,
    #[msg("Invalid input parameters.")]
    InvalidInput,
    #[msg("Operation failed due to insufficient funds.")]
    InsufficientFunds,
    #[msg("Token transfer failed.")]
    TransferFailed,
    #[msg("Custom error occurred.")]
    CustomError,
}
