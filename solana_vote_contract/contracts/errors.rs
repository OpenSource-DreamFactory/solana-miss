use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("The user is not whitelisted.")]
    UserNotWhitelisted,
    #[msg("The contract is currently inactive.")]
    ContractInactive,
    #[msg("Invalid vote settings provided.")]
    InvalidVoteSettings,
    #[msg("The amount of ROMA tokens provided does not match the required amount per vote.")]
    InvalidRomaAmount,
    #[msg("Failed to add votes to the Miss.")]
    FailedToAddVotes,
    #[msg("Miss not found.")]
    MissNotFound,
    #[msg("Insufficient permissions to perform this action.")]
    InsufficientPermissions,
    #[msg("Error while updating the vote settings.")]
    VoteSettingsUpdateError,
    #[msg("Error while toggling the contract status.")]
    ContractToggleError,
    #[msg("General error.")]
    GeneralError,
}
