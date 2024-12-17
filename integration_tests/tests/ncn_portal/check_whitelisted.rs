#[cfg(test)]
mod tests {
    use solana_sdk::{signature::Keypair, signer::Signer};

    use crate::fixtures::fixture::TestBuilder;

    #[tokio::test]
    async fn test_check_whitelisted_ok() {
        let fixture = TestBuilder::new().await;

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
    }
}
