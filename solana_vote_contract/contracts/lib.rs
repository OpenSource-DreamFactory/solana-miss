use anchor_lang::prelude::*;

declare_id!("EKHNHW28nZzPK8wK9RfMzvsoEhZKEnUzJpFkWSV2Wew");

#[program]
pub mod voting_contract {
    use super::*;

    pub fn vote(ctx: Context<Vote>, miss_id: String, roma_amount: u64) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let user = &ctx.accounts.user;

        require!(
            contract
                .whitelist
                .check(&user.to_account_info().key.to_string()),
            Errors::UserNotWhitelisted
        );
        require!(
            contract.vote_settings.contract_active,
            Errors::ContractInactive
        );
        require!(
            contract.vote_settings.roma_per_vote > 0,
            Errors::InvalidVoteSettings
        );

        let votes = roma_amount / contract.vote_settings.roma_per_vote;
        contract.add_votes(&miss_id, votes)?;
        emit!(VoteEvent {
            miss_id,
            roma_amount,
            vote_count: votes,
            user_address: user.to_account_info().key.to_string(),
        });
        Ok(())
    }

    pub fn set_vote_settings(
        ctx: Context<SetVoteSettings>,
        new_settings: VoteSettings,
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.vote_settings = new_settings;
        Ok(())
    }

    pub fn toggle_contract(ctx: Context<ToggleContract>, is_active: bool) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        contract.vote_settings.contract_active = is_active;
        Ok(())
    }
}

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

#[account]
pub struct Contract {
    pub whitelist: Whitelist,
    pub vote_settings: VoteSettings,
    pub misses: Vec<Miss>, // Updated to include Misses
}

impl Contract {
    pub fn add_votes(&mut self, miss_id: &String, vote_count: u64) -> Result<()> {
        let miss = self.misses.iter_mut().find(|m| &m.id == miss_id);
        match miss {
            Some(m) => {
                m.votes += vote_count;
                Ok(())
            }
            None => Err(Errors::MissNotFound.into()),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Whitelist {
    // Whitelist details
}

impl Whitelist {
    pub fn check(&self, address: &String) -> bool {
        // Check if an address is whitelisted
        // This should be replaced with actual logic to verify if the address is in the whitelist
        true
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VoteSettings {
    pub roma_per_vote: u64,
    pub contract_active: bool,
}

#[account]
pub struct Miss {
    pub id: String,
    pub wallet: String,
    pub votes: u64,
}

#[event]
pub struct VoteEvent {
    pub miss_id: String,
    pub roma_amount: u64,
    pub vote_count: u64,
    pub user_address: String,
}

#[error_code]
pub enum Errors {
    #[msg("The user is not whitelisted.")]
    UserNotWhitelisted,
    #[msg("The contract is currently inactive.")]
    ContractInactive,
    #[msg("Invalid vote settings.")]
    InvalidVoteSettings,
    #[msg("Miss not found.")]
    MissNotFound, // Added MissNotFound error
}
