use crate::error::CustomError;
use crate::state::{State, RomaTokenAccount, ExchangeRate};
use crate::utils::{calculate_roma_amount, transfer_tokens};
use anchor_lang::prelude::*;

pub struct Processor;

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // Unpack and validate the instruction data
        let instruction = Self::unpack_instruction(instruction_data)?;

        match instruction {
            Instruction::SetExchangeRate { rate } => {
                // Validate the number of accounts and that the first account is a signer
                Self::validate_accounts(accounts, 1, true)?;
                Self::process_set_exchange_rate(accounts, rate, program_id)
            },
            Instruction::ToggleSwap => {
                // Validate the number of accounts and that the first account is a signer
                Self::validate_accounts(accounts, 1, true)?;
                Self::process_toggle_swap(accounts, program_id)
            },
            Instruction::Transfer { amount } => {
                // Validate the number of accounts and that the first account is a signer
                Self::validate_accounts(accounts, 3, true)?;
                Self::process_transfer(accounts, amount, program_id)
            },
        }
    }

    fn process_set_exchange_rate(
        accounts: &[AccountInfo],
        rate: ExchangeRate,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account = State::try_from_slice(&accounts[0].data.borrow())?;
        account.set_exchange_rate(rate, program_id, &accounts[0])?;
        Ok(())
    }

    fn process_toggle_swap(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account = State::try_from_slice(&accounts[0].data.borrow())?;
        account.toggle_swap(program_id, &accounts[0])?;
        Ok(())
    }

    fn process_transfer(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let sender_account = RomaTokenAccount::try_from_slice(&accounts[0].data.borrow())?;
        let receiver_account = RomaTokenAccount::try_from_slice(&accounts[1].data.borrow())?;
        let state = State::try_from_slice(&accounts[2].data.borrow())?;

        let roma_amount = calculate_roma_amount(amount, state.rate)?;
        transfer_tokens(
            program_id,
            &sender_account,
            &receiver_account,
            roma_amount,
            &accounts[0],
            &accounts[1],
        )?;
        Ok(())
    }

    fn unpack_instruction(data: &[u8]) -> Result<Instruction, ProgramError> {
        Instruction::try_from_slice(data).or_else(|e| {
            msg!("Error unpacking instruction: {:?}", e);
            Err(ProgramError::CustomError(CustomError::InvalidInstruction.into()))
        })
    }

    fn validate_accounts(accounts: &[AccountInfo], expected_len: usize, needs_signer: bool) -> ProgramResult {
        if accounts.len() < expected_len {
            return Err(ProgramError::CustomError(CustomError::InvalidAccountInput.into()));
        }
        if needs_signer && !accounts[0].is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    SetExchangeRate { rate: ExchangeRate },
    ToggleSwap,
    Transfer { amount: u64 },
}
