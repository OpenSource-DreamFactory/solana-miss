use solana_program::{
    pubkey::Pubkey,
    program_error::ProgramError,
};

/// Performs a safe addition, returning an error if overflow occurs.
pub fn safe_add(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_add(b).ok_or(ProgramError::InvalidInstructionData)
}

/// Performs a safe subtraction, returning an error if underflow occurs.
pub fn safe_sub(a: u64, b: u64) -> Result<u64, ProgramError> {
    a.checked_sub(b).ok_or(ProgramError::InvalidInstructionData)
}

/// Validates if a given pubkey is not the default Pubkey.
pub fn validate_pubkey(pubkey: &Pubkey) -> Result<(), ProgramError> {
    if *pubkey == Pubkey::default() {
        Err(ProgramError::InvalidAccountData)
    } else {
        Ok(())
    }
}