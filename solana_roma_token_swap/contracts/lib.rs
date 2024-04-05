use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

pub mod processor;
pub mod state;
pub mod error;
pub mod utils;

declare_id!("Fg6PaFzntrfVLp7uLy..."); // Example Program ID, replace with actual ID

#[program]
pub mod exchange_contract {
    use super::*;

    pub fn exchange(ctx: Context<Exchange>, token_a: Pubkey, token_b: Pubkey, amount: u64) -> Result<()> {
        processor::process_exchange(ctx, token_a, token_b, amount)
    }
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub token_a_account: Account<'info, state::TokenAccount>,
    #[account(mut)]
    pub token_b_account: Account<'info, state::TokenAccount>,
    // Additional accounts can be defined here as needed
}

// Error handling
#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message.")]
    CustomError,
}