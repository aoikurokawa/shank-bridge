use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{AccountDeserialize, Discriminator};
use ncn_portal_sdk::error::NcnPortalError;
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::discriminators::Discriminators;

/// The "base" whitelist account upon which all whitelist entry account addresses are derived
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct Whitelist {
    // The account that created this whitelist
    pub admin: Pubkey,
}

impl Discriminator for Whitelist {
    const DISCRIMINATOR: u8 = Discriminators::Whitelist as u8;
}

impl Whitelist {
    pub fn new(admin: Pubkey) -> Self {
        Self { admin }
    }

    pub fn check_admin(&self, admin_info: &Pubkey) -> Result<(), NcnPortalError> {
        if self.admin.eq(admin_info) {
            Ok(())
        } else {
            Err(NcnPortalError::NcnPortalWhitelistAdminInvalid)
        }
    }

    pub fn seeds(admin: &Pubkey) -> Vec<Vec<u8>> {
        vec![b"whitelist".to_vec(), admin.to_bytes().to_vec()]
    }

    pub fn find_program_address(program_id: &Pubkey, admin: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(admin);
        let (address, bump) = Pubkey::find_program_address(
            &seeds.iter().map(|s| s.as_slice()).collect::<Vec<_>>(),
            program_id,
        );
        (address, bump, seeds)
    }

    pub fn load(
        program_id: &Pubkey,
        admin: &Pubkey,
        whitelist_account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if whitelist_account.owner.ne(program_id) {
            msg!("Whitelist account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if whitelist_account.data_is_empty() {
            msg!("Whitelist account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !whitelist_account.is_writable {
            msg!("Whitelist account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if whitelist_account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Whitelist account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if whitelist_account
            .key
            .ne(&Self::find_program_address(program_id, admin).0)
        {
            msg!("Whitelist account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
