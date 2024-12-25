mod add_to_whitelist;
mod admin_update_merkle_root;
mod check_whitelisted;
mod initialize_whitelist;
mod remove_from_whitelist;

use add_to_whitelist::process_add_to_whitelist;
use admin_update_merkle_root::process_admin_update_merkle_root;
use borsh::BorshDeserialize;
use check_whitelisted::process_check_whitelisted;
use const_str_to_pubkey::str_to_pubkey;
use initialize_whitelist::process_initialize_whitelist;
use ncn_portal_sdk::instruction::NcnPortalInstruction;
use remove_from_whitelist::process_remove_from_whitelist;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

declare_id!(str_to_pubkey(env!("NCN_PORTAL_PROGRAM_ID")));

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = NcnPortalInstruction::try_from_slice(instruction_data)?;

    match instruction {
        NcnPortalInstruction::InitializeWhitelist { root } => {
            msg!("Instruction: InitializeWhitelist");
            process_initialize_whitelist(program_id, accounts, root)
        }
        NcnPortalInstruction::AdminUpdateMerkleRoot { root } => {
            msg!("Instruction: AdminUpdateMerkleRoot");
            process_admin_update_merkle_root(program_id, accounts, root)
        }
        NcnPortalInstruction::AddToWhitelist { rate_limiting } => {
            msg!("Instruction: AddToWhitelist");
            process_add_to_whitelist(program_id, accounts, rate_limiting)
        }
        NcnPortalInstruction::CheckWhitelisted { proof } => {
            msg!("Instruction: CheckWhitelisted");
            process_check_whitelisted(program_id, accounts, proof)
        }
        NcnPortalInstruction::RemoveFromWhitelist => {
            msg!("Instruction: RemoveFromWhitelist");
            process_remove_from_whitelist(program_id, accounts)
        }
    }
}
