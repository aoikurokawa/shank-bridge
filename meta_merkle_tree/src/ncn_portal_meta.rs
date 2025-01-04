use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::pubkey_string_conversion;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct NcnPortalMeta {
    /// Rate Limiting
    rate_limiting: u64,
    // #[serde(with = "pubkey_string_conversion")]
    // pub validator_vote_account: Pubkey,
    // #[serde(with = "pubkey_string_conversion")]
    // pub validator_node_pubkey: Pubkey,

    // The validator's tip-distribution meta if it exists.
    // pub maybe_tip_distribution_meta: Option<TipDistributionMeta>,
    // Delegations to this validator.
    // pub delegations: Vec<Delegation>,

    // The total amount of delegations to the validator.
    // pub total_delegated: u64,

    // The validator's delegation commission rate as a percentage between 0-100.
    // pub commission: u8,
}
