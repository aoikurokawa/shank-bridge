use jito_bytemuck::AccountDeserialize;
use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
use ncn_portal_sdk::sdk::{
    add_to_whitelist, check_whitelisted, initialize_whitelist, remove_from_whitelist,
};
use solana_program_test::BanksClient;
use solana_sdk::{
    commitment_config::CommitmentLevel, native_token::sol_to_lamports, pubkey::Pubkey,
    signature::Keypair, signer::Signer, system_instruction::transfer, transaction::Transaction,
};

use super::TestResult;

pub struct NcnPortalProgramClient {
    banks_client: BanksClient,
    payer: Keypair,
}

impl NcnPortalProgramClient {
    pub fn new(banks_client: BanksClient, payer: Keypair) -> Self {
        Self {
            banks_client,
            payer,
        }
    }

    pub async fn get_whitelist(&mut self, account: &Pubkey) -> TestResult<Whitelist> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*Whitelist::try_from_slice_unchecked(
            account.data.as_slice(),
        )?)
    }

    pub async fn do_initialize_whitelist(&mut self, root: &[u8; 32]) -> TestResult<Keypair> {
        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;
        let admin = Keypair::new();

        self._airdrop(&admin.pubkey(), 1.0).await?;
        self.initialize_whitelist(&whitelist_pubkey, &admin, root)
            .await?;

        Ok(admin)
    }

    pub async fn initialize_whitelist(
        &mut self,
        whitelist: &Pubkey,
        admin: &Keypair,
        root: &[u8; 32],
    ) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[initialize_whitelist(
                &ncn_portal_program::id(),
                whitelist,
                &admin.pubkey(),
                *root,
            )],
            Some(&admin.pubkey()),
            &[admin],
            blockhash,
        ))
        .await
    }

    pub async fn get_whitelist_entry(&mut self, account: &Pubkey) -> TestResult<WhitelistEntry> {
        let account = self.banks_client.get_account(*account).await?.unwrap();
        Ok(*WhitelistEntry::try_from_slice_unchecked(
            account.data.as_slice(),
        )?)
    }

    pub async fn do_add_to_whitelist(
        &mut self,
        whitelisted: &Pubkey,
        admin: &Keypair,
        rate_limiting: u64,
    ) -> TestResult<()> {
        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;
        let whitelist_entry_pubkey = WhitelistEntry::find_program_address(
            &ncn_portal_program::id(),
            &whitelist_pubkey,
            whitelisted,
        )
        .0;

        self._airdrop(&admin.pubkey(), 1.0).await?;
        self.add_to_whitelist(
            &whitelist_pubkey,
            &whitelist_entry_pubkey,
            whitelisted,
            &admin,
            rate_limiting,
        )
        .await?;

        Ok(())
    }

    pub async fn add_to_whitelist(
        &mut self,
        whitelist: &Pubkey,
        whitelist_entry: &Pubkey,
        whitelisted: &Pubkey,
        admin: &Keypair,
        rate_limiting: u64,
    ) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[add_to_whitelist(
                &ncn_portal_program::id(),
                whitelist,
                whitelist_entry,
                whitelisted,
                &admin.pubkey(),
                rate_limiting,
            )],
            Some(&admin.pubkey()),
            &[admin],
            blockhash,
        ))
        .await
    }

    pub async fn do_check_whitelisted(
        &mut self,
        whitelisted: &Keypair,
        proof: Vec<[u8; 32]>,
    ) -> TestResult<()> {
        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;

        self._airdrop(&whitelisted.pubkey(), 1.0).await?;

        self.check_whitelisted(&whitelist_pubkey, whitelisted, proof)
            .await?;

        Ok(())
    }

    pub async fn check_whitelisted(
        &mut self,
        whitelist: &Pubkey,
        whitelisted: &Keypair,
        proof: Vec<[u8; 32]>,
    ) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[check_whitelisted(
                &ncn_portal_program::id(),
                whitelist,
                &whitelisted.pubkey(),
                proof,
            )],
            Some(&whitelisted.pubkey()),
            &[whitelisted],
            blockhash,
        ))
        .await
    }

    pub async fn do_remove_from_whitelist(
        &mut self,
        whitelisted: &Pubkey,
        admin: &Keypair,
    ) -> TestResult<()> {
        let whitelist_pubkey = Whitelist::find_program_address(&ncn_portal_program::id()).0;
        let whitelist_entry_pubkey = WhitelistEntry::find_program_address(
            &ncn_portal_program::id(),
            &whitelist_pubkey,
            whitelisted,
        )
        .0;

        self.remove_from_whitelist(
            &whitelist_pubkey,
            &whitelist_entry_pubkey,
            whitelisted,
            admin,
        )
        .await?;

        Ok(())
    }

    pub async fn remove_from_whitelist(
        &mut self,
        whitelist: &Pubkey,
        whitelist_entry: &Pubkey,
        whitelisted: &Pubkey,
        admin: &Keypair,
    ) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.process_transaction(&Transaction::new_signed_with_payer(
            &[remove_from_whitelist(
                &ncn_portal_program::id(),
                whitelist,
                whitelist_entry,
                whitelisted,
                &admin.pubkey(),
            )],
            Some(&admin.pubkey()),
            &[admin],
            blockhash,
        ))
        .await
    }

    pub async fn process_transaction(&mut self, tx: &Transaction) -> TestResult<()> {
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                tx.clone(),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }

    pub async fn _airdrop(&mut self, to: &Pubkey, sol: f64) -> TestResult<()> {
        let blockhash = self.banks_client.get_latest_blockhash().await?;
        self.banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(&self.payer.pubkey(), to, sol_to_lamports(sol))],
                    Some(&self.payer.pubkey()),
                    &[&self.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await?;
        Ok(())
    }
}
