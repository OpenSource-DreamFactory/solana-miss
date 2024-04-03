## state.rs
use anchor_lang::prelude::*;

/// Represents an account holding ROMA tokens.
#[account]
pub struct RomaTokenAccount {
    pub owner: Pubkey,
    pub amount: u64,
}

/// Represents the exchange rate between ROMA tokens and SOL.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ExchangeRate {
    pub rate: u64, // The rate is represented as the amount of ROMA tokens per SOL token.
}

impl ExchangeRate {
    const DEFAULT_EXCHANGE_RATE: u64 = 100; // Assuming 1 SOL = 100 ROMA as a default exchange rate

    // Sets a default value for ExchangeRate
    pub fn default() -> Self {
        Self { rate: Self::DEFAULT_EXCHANGE_RATE } // Using constant for default exchange rate
    }
}

/// Represents the state of the ROMA token exchange platform, including the token account, exchange rate, and swap status.
#[account]
pub struct State {
    pub roma_token_account: RomaTokenAccount,
    pub exchange_rate: ExchangeRate,
    pub swap_enabled: bool,
}

impl State {
    /// Initializes the State with default values.
    pub fn new(owner: Pubkey, amount: u64) -> Self {
        Self {
            roma_token_account: RomaTokenAccount { owner, amount },
            exchange_rate: ExchangeRate::default(),
            swap_enabled: true, // By default, swapping is enabled
        }
    }

    /// Sets the exchange rate for ROMA token.
    pub fn set_exchange_rate(&mut self, rate: u64) {
        self.exchange_rate.rate = rate;
    }

    /// Toggles the swap functionality on or off.
    pub fn toggle_swap(&mut self) {
        self.swap_enabled = !self.swap_enabled;
    }
}
