use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_error::ProgramError;
use std::collections::HashSet;

#[account]
pub struct Contract {
    pub whitelist: Pubkey,
    pub vote_settings: VoteSettings,
}

impl Contract {
    pub fn get_votes(&self, miss_id: String) -> Result<u64, ProgramError> {
        // This method should interact with the Solana blockchain to fetch the Miss account based on the ID
        // and return its vote count. The actual implementation will depend on the broader application architecture.
        // Placeholder logic for demonstration.
        Err(ProgramError::Custom(0)) // Custom error indicating the function is not implemented
    }
}

#[account]
pub struct Whitelist {
    // This struct now contains a HashSet of Pubkeys representing whitelisted addresses.
    pub whitelisted_addresses: HashSet<Pubkey>,
}

impl Whitelist {
    pub fn check(&self, address: Pubkey) -> bool {
        // This method now searches through the whitelist data to check if an address is whitelisted.
        self.whitelisted_addresses.contains(&address)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct VoteSettings {
    pub roma_per_vote: u64,
    pub contract_active: bool,
}

impl VoteSettings {
    pub fn update_settings(&mut self, new_settings: VoteSettings) {
        self.roma_per_vote = new_settings.roma_per_vote;
        self.contract_active = new_settings.contract_active;
    }
}

#[account]
pub struct Miss {
    pub id: String,
    pub wallet: Pubkey,
    pub votes: u64,
}

impl Miss {
    pub fn add_votes(&mut self, vote_count: u64) {
        self.votes += vote_count;
    }
}
