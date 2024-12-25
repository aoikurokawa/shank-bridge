#[cfg(test)]
mod tests {
    use meta_merkle_tree::{meta_merkle_tree::MetaMerkleTree, tree_node::TreeNode};
    use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
    use solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signer},
    };

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_remove_from_whitelist_ok() {
        let mut fixture = TestBuilder::new().await;

        let mut ncn_portal_program_client = fixture.ncn_portal_program_client();

        let alice = Pubkey::new_unique();

        let tree_nodes = vec![TreeNode::new(&alice, 0)];
        let merkle_info = MetaMerkleTree::new(tree_nodes).unwrap();

        let admin = ncn_portal_program_client
            .do_initialize_whitelist(&merkle_info.merkle_root)
            .await
            .unwrap();

        let whitelisted = Keypair::new();
        let rate_limiting = 10;

        ncn_portal_program_client
            .do_add_to_whitelist(&whitelisted.pubkey(), &admin, rate_limiting)
            .await
            .unwrap();

        let proof = &merkle_info.tree_nodes[0].proof.clone().unwrap().clone();

        ncn_portal_program_client
            .do_check_whitelisted(&whitelisted, proof.clone())
            .await
            .unwrap();

        ncn_portal_program_client
            .do_remove_from_whitelist(&whitelisted.pubkey(), &admin)
            .await
            .unwrap();

        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;
        let whitelist_entry_pubkey = WhitelistEntry::find_program_address(
            &ncn_portal_program::id(),
            &whitelist_pubkey,
            &whitelisted.pubkey(),
        )
        .0;

        let whitelist_entry_acc = fixture
            .context
            .banks_client
            .get_account(whitelist_entry_pubkey)
            .await
            .unwrap();
        assert!(whitelist_entry_acc.is_none());
    }
}
