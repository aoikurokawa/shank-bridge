use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum NcnPortalInstruction {
    /// Initializes global configuration
    #[account(0, writable, name = "whitelist")]
    #[account(1, writable, signer, name = "admin")]
    #[account(2, name = "system_program")]
    InitializeWhitelist,

    /// Initializes global configuration
    #[account(0, name = "whitelist")]
    #[account(1, writable, name = "whitelist_entry")]
    #[account(2, name = "whitelisted")]
    #[account(3, writable, signer, name = "admin")]
    #[account(4, name = "system_program")]
    AddToWhitelist { rate_limiting: u64 },

    /// Check Whitelist
    #[account(0, name = "whitelist")]
    #[account(1, name = "whitelist_entry")]
    #[account(2, signer, name = "whitelisted")]
    CheckWhitelisted,

    /// Removed from Whitelist
    #[account(0, name = "whitelist")]
    #[account(1, writable, name = "whitelist_entry")]
    #[account(2, signer, name = "admin")]
    #[account(3, name = "system_program")]
    RemoveFromWhitelist,

    /// Set RateLimiting
    #[account(0, name = "whitelist")]
    #[account(1, writable, name = "whitelist_entry")]
    #[account(2, signer, name = "admin")]
    SetRateLimiting { rate_limiting: u64 },
}
