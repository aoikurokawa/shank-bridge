use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use ncn_portal_core::{whitelist::Whitelist, whitelist_entry::WhitelistEntry};
use ncn_portal_sdk::error::NcnPortalError;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar,
};

pub fn process_add_to_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    rate_limiting: u64,
) -> ProgramResult {
    let [whitelist_info, whitelist_entry_info, whitelisted_info, admin_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Whitelist::load(program_id, whitelist_info, false)?;
    let whitelist_data = whitelist_info.data.borrow();
    let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;

    whitelist.check_admin(admin_info.key)?;

    load_system_account(whitelist_entry_info, true)?;
    load_signer(admin_info, true)?;
    load_system_program(system_program)?;

    // The whitelist entry account shall be at the canonical PDA
    let (whitelist_entry_pubkey, whitelist_entry_bump, mut whitelist_entry_seeds) =
        WhitelistEntry::find_program_address(program_id, whitelist_info.key, whitelisted_info.key);
    whitelist_entry_seeds.push(vec![whitelist_entry_bump]);
    if whitelist_entry_pubkey.ne(whitelist_entry_info.key) {
        msg!("Whitelist entry account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!(
        "Initializing whitelist entry at address {}",
        whitelist_entry_info.key
    );
    create_account(
        admin_info,
        whitelist_entry_info,
        system_program,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(std::mem::size_of::<WhitelistEntry>() as u64)
            .ok_or(NcnPortalError::ArithmeticOverflow)?,
        &whitelist_entry_seeds,
    )?;

    let mut whitelist_entry_data = whitelist_entry_info.try_borrow_mut_data()?;
    whitelist_entry_data[0] = WhitelistEntry::DISCRIMINATOR;
    let whitelist_entry = WhitelistEntry::try_from_slice_unchecked_mut(&mut whitelist_entry_data)?;
    *whitelist_entry =
        WhitelistEntry::new(*whitelist_info.key, *whitelisted_info.key, rate_limiting);

    Ok(())
}
