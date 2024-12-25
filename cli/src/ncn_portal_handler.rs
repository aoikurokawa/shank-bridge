use anyhow::{anyhow, Result};
use jito_bytemuck::AccountDeserialize;
use log::{debug, info};
use ncn_portal_client::instructions::RemoveFromWhitelistBuilder;
use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
use ncn_portal_sdk::sdk::{add_to_whitelist, admin_update_merkle_tree, initialize_whitelist};
use solana_program::pubkey::Pubkey;
use solana_rpc_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction};
use solana_sdk::{signature::Signer, transaction::Transaction};

use crate::{
    ncn_portal::{NcnPortalCommands, WhitelistActions},
    CliConfig,
};

pub struct NcnPortalCliHandler {
    cli_config: CliConfig,
    ncn_portal_program_id: Pubkey,
}

impl NcnPortalCliHandler {
    pub const fn new(cli_config: CliConfig, ncn_portal_program_id: Pubkey) -> Self {
        Self {
            cli_config,
            ncn_portal_program_id,
        }
    }

    fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.cli_config.rpc_url.clone(), self.cli_config.commitment)
    }

    pub async fn handle(&self, action: NcnPortalCommands) -> Result<()> {
        match action {
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::Initialize,
            } => self.initialize_whitelist().await,
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::Get,
            } => self.get_whitelist().await,
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::AdminUpdateMerkleRoot { root },
            } => self.admint_update_merkle_root(root).await,
            NcnPortalCommands::Whitelist {
                action:
                    WhitelistActions::AddToWhitelist {
                        whitelisted,
                        rate_limiting,
                    },
            } => self.add_to_whitelist(whitelisted, rate_limiting).await,
            NcnPortalCommands::Whitelist {
                action: WhitelistActions::RemoveFromWhitelist { whitelisted },
            } => self.remove_from_whitelist(whitelisted).await,
        }
    }

    async fn initialize_whitelist(&self) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;

        let ix = initialize_whitelist(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &keypair.pubkey(),
            [0u8; 32],
        );
        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &[keypair],
            blockhash,
        );
        info!(
            "Initializing whitelist transaction: {:?}",
            tx.get_signature()
        );
        rpc_client.send_and_confirm_transaction(&tx).await?;
        info!("Transaction confirmed: {:?}", tx.get_signature());
        Ok(())
    }

    async fn get_whitelist(&self) -> Result<()> {
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;
        debug!(
            "Reading the whitelist account at address: {}",
            whitelist_address
        );

        let account = rpc_client.get_account(&whitelist_address).await?;
        let whitelist = Whitelist::try_from_slice_unchecked(&account.data)?;
        info!(
            "Whitelist at address {}: {:?}",
            whitelist_address, whitelist
        );
        Ok(())
    }

    async fn add_to_whitelist(&self, whitelisted: Pubkey, rate_limiting: u64) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;
        let whitelist_entry_address = WhitelistEntry::find_program_address(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &whitelisted,
        )
        .0;

        let ix = add_to_whitelist(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &whitelist_entry_address,
            &whitelisted,
            &keypair.pubkey(),
            rate_limiting,
        );

        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &[keypair],
            blockhash,
        );
        info!("Adding To Whitelist transaction: {:?}", tx.get_signature());
        let result = rpc_client.send_and_confirm_transaction(&tx).await?;
        info!("Transaction confirmed: {:?}", result);
        let statuses = rpc_client
            .get_signature_statuses(&[*tx.get_signature()])
            .await?;

        let tx_status = statuses
            .value
            .first()
            .unwrap()
            .as_ref()
            .ok_or_else(|| anyhow!("No signature status"))?;
        info!("Transaction status: {:?}", tx_status);

        Ok(())
    }

    async fn admint_update_merkle_root(&self, merkle_root: Vec<u8>) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;

        let mut root = [0u8; 32];
        let len = merkle_root.len().min(32);
        root[..len].copy_from_slice(&merkle_root[..len]);

        let ix = admin_update_merkle_tree(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &keypair.pubkey(),
            root,
        );

        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&keypair.pubkey()),
            &[keypair],
            blockhash,
        );
        info!("Updating Whitelist transaction: {:?}", tx.get_signature());
        let result = rpc_client.send_and_confirm_transaction(&tx).await?;
        info!("Transaction confirmed: {:?}", result);
        let statuses = rpc_client
            .get_signature_statuses(&[*tx.get_signature()])
            .await?;

        let tx_status = statuses
            .value
            .first()
            .unwrap()
            .as_ref()
            .ok_or_else(|| anyhow!("No signature status"))?;
        info!("Transaction status: {:?}", tx_status);

        Ok(())
    }

    async fn remove_from_whitelist(&self, whitelisted: Pubkey) -> Result<()> {
        let keypair = self
            .cli_config
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair"))?;
        let rpc_client = self.get_rpc_client();

        let whitelist_address = Whitelist::find_program_address(&self.ncn_portal_program_id).0;
        let whitelist_entry_address = WhitelistEntry::find_program_address(
            &self.ncn_portal_program_id,
            &whitelist_address,
            &whitelisted,
        )
        .0;

        let mut ix_builder = RemoveFromWhitelistBuilder::new();
        ix_builder
            .whitelist(whitelist_address)
            .whitelist_entry(whitelist_entry_address)
            .whitelisted_info(whitelisted)
            .admin_info(keypair.pubkey())
            .instruction();

        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix_builder.instruction()],
            Some(&keypair.pubkey()),
            &[keypair],
            blockhash,
        );
        info!(
            "Removing from Whitelist transaction: {:?}",
            tx.get_signature()
        );
        let result = rpc_client.send_and_confirm_transaction(&tx).await?;
        info!("Transaction confirmed: {:?}", result);
        let statuses = rpc_client
            .get_signature_statuses(&[*tx.get_signature()])
            .await?;

        let tx_status = statuses
            .value
            .first()
            .unwrap()
            .as_ref()
            .ok_or_else(|| anyhow!("No signature status"))?;
        info!("Transaction status: {:?}", tx_status);

        Ok(())
    }
}
