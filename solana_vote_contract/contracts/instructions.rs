use anchor_lang::prelude::*;
use crate::state::{Contract, VoteSettings};
use crate::errors::VotingError;

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub contract: Account<'info, Contract>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetVoteSettings<'info> {
    #[account(mut)]
    pub contract: Account<'info, Contract>,
}

#[derive(Accounts)]
pub struct ToggleContract<'info> {
    #[account(mut)]
    pub contract: Account<'info, Contract>,
}

impl Contract {
    // Encapsulated method for whitelist check, streamlined with Result::ok_or_else
    fn is_user_whitelisted(&self, user_address: &str) -> Result<()> {
        self.whitelist.check(user_address)
            .then(|| ())
            .ok_or_else(|| VotingError::UserNotWhitelisted.into())
    }

    // Encapsulated method for voting settings check, streamlined with Result::ok_or_else
    fn is_voting_active(&self) -> Result<()> {
        self.vote_settings.contract_active
            .then(|| ())
            .ok_or_else(|| VotingError::VotingNotActive.into())
    }

    /// Processes a vote for a Miss, performing necessary checks and calculations.
    /// Emits a VoteEvent upon successful vote.
    pub fn vote(ctx: Context<Vote>, miss_id: String, roma_amount: u64) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let user_address = ctx.accounts.user.key().to_string();

        // Use encapsulated methods for checks
        contract.is_user_whitelisted(&user_address)?;
        contract.is_voting_active()?;

        // Calculate the number of votes based on the roma_amount and roma_per_vote
        let calculated_votes = roma_amount / contract.vote_settings.roma_per_vote;

        // Add votes to the Miss
        contract.add_votes(miss_id.clone(), calculated_votes)?;

        // Emitting an event for the vote
        emit!(VoteEvent {
            miss_id,
            miss_wallet: contract.get_miss_wallet(&miss_id)?,
            user_address,
            roma_amount,
            vote_count: calculated_votes,
        });

        Ok(())
    }

    /// Updates the voting settings of the contract.
    pub fn set_vote_settings(ctx: Context<SetVoteSettings>, new_settings: VoteSettings) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.vote_settings = new_settings;
        Ok(())
    }

    /// Toggles the active status of the contract.
    pub fn toggle_contract(ctx: Context<ToggleContract>, is_active: bool) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.vote_settings.contract_active = is_active;
        Ok(())
    }
}

#[event]
pub struct VoteEvent {
    pub miss_id: String,
    pub miss_wallet: String,
    pub user_address: String,
    pub roma_amount: u64,
    pub vote_count: u64,
}
