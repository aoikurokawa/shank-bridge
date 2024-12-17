use jito_jsm_core::{
    close_program_account,
    loader::{load_signer, load_system_program},
};
use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_remove_from_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [whitelist_info, whitelist_entry_info, whitelisted_info, admin_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Whitelist::load(program_id, whitelist_info, false)?;
    WhitelistEntry::load(
        program_id,
        whitelist_info.key,
        whitelisted_info.key,
        whitelist_entry_info,
        false,
    )?;

    load_signer(admin_info, false)?;
    load_system_program(system_program)?;

    close_program_account(program_id, whitelist_entry_info, admin_info)?;

    Ok(())
}
