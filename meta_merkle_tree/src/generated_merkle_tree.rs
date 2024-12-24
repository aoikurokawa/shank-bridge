// Mostly copied from modules in jito-solana/tip-distributor/src
// To be replaced by tip distributor code in this repo

use serde::{Deserialize, Serialize};
use solana_program::{hash::Hash, pubkey::Pubkey};

use crate::{pubkey_string_conversion, stake_meta_tree_node::StakeMetaTreeNode};

#[derive(Clone, Eq, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct GeneratedMerkleTree {
    #[serde(with = "pubkey_string_conversion")]
    pub tip_distribution_account: Pubkey,

    #[serde(with = "pubkey_string_conversion")]
    pub merkle_root_upload_authority: Pubkey,

    pub merkle_root: Hash,

    pub tree_nodes: Vec<StakeMetaTreeNode>,

    // pub max_total_claim: u64,
    pub max_num_nodes: u64,
}
