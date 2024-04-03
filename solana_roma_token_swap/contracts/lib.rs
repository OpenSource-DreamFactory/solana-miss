use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
mod state;
mod utils;
use crate::error::Error;
mod processor;

declare_id!("EKHNHW28nZzPK8wK9RfMzvsoEhZKEnUzJpFkWSV2Wew");

#[program]
pub mod roma_token_exchange {
    use super::*;
    use anchor_spl::token::{self, Token, TokenAccount, Transfer};

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.set_exchange_rate(1.0)?;
        Ok(())
    }

    pub fn toggle_swap(ctx: Context<ToggleSwap>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.toggle_swap()?;
        Ok(())
    }

    pub fn exchange_roma(ctx: Context<ExchangeRoma>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;
        let processor = Processor {};
        processor.process_instruction(state, amount)?;
        // Directly interact with Solana's token program for transferring tokens
        token::transfer(ctx.accounts.transfer_context(), amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ToggleSwap<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
}

#[derive(Accounts)]
pub struct ExchangeRoma<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub user: Signer<'info>,
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ExchangeRoma<'info> {
    // Context for token transfer
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.from.to_account_info().clone(),
            to: self.to.to_account_info().clone(),
            authority: self.user.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        CpiContext::new(cpi_program, cpi_accounts)
    }
}
