mod delete_slash_proposal;
mod execute_slash;
mod initialize_config;
mod initialize_ncn_resolver_program_config;
mod initialize_resolver;
mod initialize_slasher;
mod propose_slash;
mod set_resolver;
mod slasher_delegate_token_account;
mod slasher_set_admin;
mod slasher_set_secondary_admin;
mod veto_slash;

use borsh::BorshDeserialize;
use delete_slash_proposal::process_delete_slash_proposal;
use resolver_sdk::instruction::ResolverInstruction;
use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

use crate::{
    execute_slash::process_execute_slash, initialize_config::process_initialize_config,
    initialize_ncn_resolver_program_config::process_initialize_resolver_program_config,
    initialize_resolver::process_initialize_resolver,
    initialize_slasher::process_initialize_slasher, propose_slash::process_propose_slash,
    set_resolver::process_set_resolver,
    slasher_delegate_token_account::process_slasher_delegate_token_account,
    slasher_set_admin::process_slasher_set_admin,
    slasher_set_secondary_admin::process_slasher_set_secondary_admin,
    veto_slash::process_veto_slash,
};

declare_id!("AE7fSUJSGxMzjNxSPpNTemrz9cr26RFue4GwoJ1cuR6f");

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = ResolverInstruction::try_from_slice(instruction_data)?;

    match instruction {
        ResolverInstruction::InitializeConfig => {
            msg!("Instruction: InitializeConfig");
            process_initialize_config(program_id, accounts)?;
        }

        ResolverInstruction::InitializeNcnResolverProgramConfig {
            veto_duration,
            delete_slash_proposal_duration,
        } => {
            msg!("Instruction: InitializeNcnResolverProgramConfig");
            process_initialize_resolver_program_config(
                program_id,
                accounts,
                veto_duration,
                delete_slash_proposal_duration,
            )?;
        }

        ResolverInstruction::InitializeResolver => {
            msg!("Instruction: InitializeResolver");
            process_initialize_resolver(program_id, accounts)?;
        }

        ResolverInstruction::InitializeSlasher => {
            msg!("Instruction: InitializeSlasher");
            process_initialize_slasher(program_id, accounts)?;
        }

        ResolverInstruction::ProposeSlash { slash_amount } => {
            msg!("Instruction: ProposeSlash");
            process_propose_slash(program_id, accounts, slash_amount)?;
        }

        ResolverInstruction::SetResolver => {
            msg!("Instruction: SetResolver");
            process_set_resolver(program_id, accounts)?;
        }

        ResolverInstruction::VetoSlash => {
            msg!("Instruction: VetoSlash");
            process_veto_slash(program_id, accounts)?;
        }

        ResolverInstruction::ExecuteSlash => {
            msg!("Instruction: ExecuteSlash");
            process_execute_slash(program_id, accounts)?;
        }

        ResolverInstruction::SlasherDelegateTokenAccount => {
            msg!("Instruction: ExecuteSlash");
            process_slasher_delegate_token_account(program_id, accounts)?;
        }

        ResolverInstruction::SlasherSetAdmin => {
            msg!("Instruction: SlasherSetAdmin");
            process_slasher_set_admin(program_id, accounts)?;
        }

        ResolverInstruction::SlasherSetSecondaryAdmin(role) => {
            msg!("Instruction: SlasherSetSecondaryAdmin");
            process_slasher_set_secondary_admin(program_id, accounts, role)?;
        }

        ResolverInstruction::DeleteSlashProposal => {
            msg!("Instruction: DeleteSlashProposal");
            process_delete_slash_proposal(program_id, accounts)?;
        }
    }

    Ok(())
}
