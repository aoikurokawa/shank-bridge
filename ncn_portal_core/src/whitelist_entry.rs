use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use ncn_portal_sdk::error::NcnPortalError;
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::discriminators::Discriminators;

/// a PDA derived from the address of the account to add and the base whitelist
/// defined in create_whitelist::Whitelist
///
/// Checking if an account address X is whitelisted in whitelist Y
/// involves checking if a WhitelistEntry exists whose address is derived from X and Y
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct WhitelistEntry {
    /// The base whitelist account that this entry is derived from
    pub parent: Pubkey,

    /// The address that this entry whitelists
    pub whitelisted: Pubkey,

    /// Rate limiting
    pub rate_limiting: PodU64,
}

impl Discriminator for WhitelistEntry {
    const DISCRIMINATOR: u8 = Discriminators::WhitelistEntry as u8;
}

impl WhitelistEntry {
    pub fn new(parent: Pubkey, whitelisted: Pubkey, rate_limiting: u64) -> Self {
        Self {
            parent,
            whitelisted,
            rate_limiting: PodU64::from(rate_limiting),
        }
    }

    pub fn check_parent(&self, parent_info: &Pubkey) -> Result<(), NcnPortalError> {
        if self.parent.eq(parent_info) {
            Ok(())
        } else {
            Err(NcnPortalError::NcnPortalParentInvalid)
        }
    }

    pub fn check_whitelisted(&self, whitelisted_info: &Pubkey) -> Result<(), NcnPortalError> {
        if self.whitelisted.eq(whitelisted_info) {
            Ok(())
        } else {
            Err(NcnPortalError::NcnPortalWhitelistedInvalid)
        }
    }

    pub fn seeds(parent: &Pubkey, whitelisted: &Pubkey) -> Vec<Vec<u8>> {
        vec![
            b"whitelist_entry".to_vec(),
            parent.to_bytes().to_vec(),
            whitelisted.to_bytes().to_vec(),
        ]
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        parent: &Pubkey,
        whitelisted: &Pubkey,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(parent, whitelisted);
        let (address, bump) = Pubkey::find_program_address(
            &seeds.iter().map(|s| s.as_slice()).collect::<Vec<_>>(),
            program_id,
        );
        (address, bump, seeds)
    }

    pub fn load(
        program_id: &Pubkey,
        parent: &Pubkey,
        whitelisted: &Pubkey,
        whitelist_entry_account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if whitelist_entry_account.owner.ne(program_id) {
            msg!("Whitelist Entry account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if whitelist_entry_account.data_is_empty() {
            msg!("Whitelist Entry account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !whitelist_entry_account.is_writable {
            msg!("Whitelist Entry account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if whitelist_entry_account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Whitelist Entry account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if whitelist_entry_account.key.ne(&Self::find_program_address(
            program_id,
            parent,
            whitelisted,
        )
        .0)
        {
            msg!("Whitelist Entry account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
