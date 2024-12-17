#[cfg(test)]
mod tests {
    use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
    use solana_sdk::signature::{Keypair, Signer};

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_remove_from_whitelist_ok() {
        let mut fixture = TestBuilder::new().await;

        let mut ncn_portal_program_client = fixture.ncn_portal_program_client();

        let admin = ncn_portal_program_client
            .do_initialize_whitelist()
            .await
            .unwrap();

        let whitelisted = Keypair::new();
        let rate_limiting = 10;

        ncn_portal_program_client
            .do_add_to_whitelist(&whitelisted.pubkey(), &admin, rate_limiting)
            .await
            .unwrap();

        ncn_portal_program_client
            .do_check_whitelisted(&whitelisted)
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
