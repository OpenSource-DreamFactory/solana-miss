## utils.rs
use anchor_lang::prelude::*;
use solana_program::program::invoke;
use solana_program::program::invoke_signed;
use solana_program::system_instruction::transfer;
use crate::error::ErrorCode;
use crate::state::ExchangeRate;

pub struct Utils;

impl Utils {
    /// Calculates the amount of ROMA tokens to be received based on the provided SOL amount and the current exchange rate.
    /// It ensures the exchange rate is not zero to prevent division by zero errors.
    ///
    /// # Arguments
    ///
    /// * `sol_amount` - The amount of SOL to be exchanged.
    /// * `exchange_rate` - The current exchange rate from SOL to ROMA.
    ///
    /// # Returns
    ///
    /// The amount of ROMA tokens to be received or an error if the exchange rate is invalid.
    pub fn calculate_roma_amount(sol_amount: u64, exchange_rate: ExchangeRate) -> Result<u64, ProgramError> {
        if exchange_rate.rate == 0 {
            Err(ErrorCode::InvalidExchangeRate.into())
        } else {
            sol_amount.checked_mul(exchange_rate.rate)
                .ok_or_else(|| ErrorCode::Overflow.into())
        }
    }

    /// Transfers tokens from one account to another. This function is designed to work with both PDA and non-PDA accounts.
    /// For non-PDA accounts, pass an empty array for `signers_seeds`.
    ///
    /// # Arguments
    ///
    /// * `from_pubkey` - The public key of the sender's account.
    /// * `to_pubkey` - The public key of the receiver's account.
    /// * `amount` - The amount of tokens to transfer.
    /// * `program_id` - The program ID of the token program.
    /// * `accounts` - The accounts required for the token transfer.
    /// * `signers_seeds` - The seeds required to sign the transaction if the from account is a PDA. Pass an empty array for non-PDA accounts.
    ///
    /// # Returns
    ///
    /// An indication of whether the transfer was successful or not.
    pub fn transfer_tokens(
        from_pubkey: &Pubkey,
        to_pubkey: &Pubkey,
        amount: u64,
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        signers_seeds: &[&[&[u8]]],
    ) -> Result<(), ProgramError> {
        let transfer_instruction = transfer(from_pubkey, to_pubkey, amount);
        let result = if signers_seeds.is_empty() {
            invoke(&transfer_instruction, accounts)
        } else {
            invoke_signed(&transfer_instruction, accounts, signers_seeds)
        };
        result.map_err(|_| ErrorCode::TokenTransferFailed.into())
    }
}
