#[cfg(test)]
mod tests {
    use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
    use solana_sdk::pubkey::Pubkey;

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_add_to_whitelist_ok() {
        let fixture = TestBuilder::new().await;

        let mut ncn_portal_program_client = fixture.ncn_portal_program_client();

        let admin = ncn_portal_program_client
            .do_initialize_whitelist()
            .await
            .unwrap();

        let whitelisted = Pubkey::new_unique();
        let rate_limiting = 10;

        ncn_portal_program_client
            .do_add_to_whitelist(&whitelisted, &admin, rate_limiting)
            .await
            .unwrap();

        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;
        let whitelist_entry_pubkey = WhitelistEntry::find_program_address(
            &ncn_portal_program::id(),
            &whitelist_pubkey,
            &whitelisted,
        )
        .0;

        let whitelist_entry = ncn_portal_program_client
            .get_whitelist_entry(&whitelist_entry_pubkey)
            .await
            .unwrap();

        assert_eq!(whitelist_entry.parent, whitelist_pubkey);
        assert_eq!(whitelist_entry.whitelisted, whitelisted);
    }
}
