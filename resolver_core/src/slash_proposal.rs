use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU64, AccountDeserialize, Discriminator};
use resolver_sdk::error::ResolverError;
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

/// The vault configuration account for the vault program.
/// Manages program-wide settings and state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct SlashProposal {
    /// The operator account
    pub operator: Pubkey,

    /// The operator account
    pub resolver: Pubkey,

    /// The slash amount
    amount: PodU64,

    pub capture_slot: PodU64,

    veto_deadline_slot: PodU64,

    pub completed: u8,

    /// The bump seed for the PDA
    pub bump: u8,
    // Reserved space
    // reserved: [u8; 263],
}

impl Discriminator for SlashProposal {
    const DISCRIMINATOR: u8 = 1;
}

impl Default for SlashProposal {
    fn default() -> Self {
        Self {
            operator: Pubkey::default(),
            resolver: Pubkey::default(),
            amount: PodU64::from(0),
            capture_slot: PodU64::from(0),
            veto_deadline_slot: PodU64::from(0),
            completed: 0,
            bump: 0,
            // reserved: [0; 263],
        }
    }
}

impl SlashProposal {
    pub fn new(
        operator: Pubkey,
        resolver: Pubkey,
        amount: u64,
        capture_slot: u64,
        veto_deadline_slot: u64,
        bump: u8,
    ) -> Self {
        Self {
            operator,
            resolver,
            amount: PodU64::from(amount),
            capture_slot: PodU64::from(capture_slot),
            veto_deadline_slot: PodU64::from(veto_deadline_slot),
            completed: 0,
            bump,
            // reserved: [0; 263],
        }
    }

    pub fn amount(&self) -> u64 {
        self.amount.into()
    }

    pub fn veto_deadline_slot(&self) -> u64 {
        self.veto_deadline_slot.into()
    }

    pub fn check_veto_period_ended(&self, current_slot: u64) -> Result<(), ResolverError> {
        if self.veto_deadline_slot() <= current_slot {
            msg!("Veto period ended");
            return Err(ResolverError::SlashProposalVetoPeriodEnded);
        }

        Ok(())
    }

    pub fn check_veto_period_not_ended(&self, current_slot: u64) -> Result<(), ResolverError> {
        if self.veto_deadline_slot() > current_slot {
            msg!("Veto period not ended");
            return Err(ResolverError::SlashProposalVetoPeriodNotEnded);
        }

        Ok(())
    }

    pub fn check_completed(&self) -> Result<(), ResolverError> {
        if self.completed.eq(&1_u8) {
            msg!("Slash proposal completed");
            return Err(ResolverError::SlashProposalCompleted);
        }

        Ok(())
    }

    pub fn seeds(ncn: &Pubkey, operator: &Pubkey, resolver: &Pubkey) -> Vec<Vec<u8>> {
        Vec::from_iter([
            b"slash_proposal".to_vec(),
            ncn.as_ref().to_vec(),
            operator.as_ref().to_vec(),
            resolver.as_ref().to_vec(),
        ])
    }

    pub fn find_program_address(
        program_id: &Pubkey,
        ncn: &Pubkey,
        operator: &Pubkey,
        resolver: &Pubkey,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(ncn, operator, resolver);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Loads the account as an [`RequestSlashList`] account, returning an error if it is not.
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// * `slash_request_list` - The account to load the SlashRequestList from
    /// * `ncn` - The account to load the SlashRequestList from
    /// * `expect_writable` - Whether the account should be writable
    ///
    /// # Returns
    /// * `Result<(), ProgramError>` - The result of the operation
    pub fn load(
        program_id: &Pubkey,
        slash_proposal: &AccountInfo,
        ncn: &AccountInfo,
        operator: &AccountInfo,
        resolver: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if slash_proposal.owner.ne(program_id) {
            msg!("SlashProposal account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if slash_proposal.data_is_empty() {
            msg!("SlashProposal account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !slash_proposal.is_writable {
            msg!("SlashProposal account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if slash_proposal.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("SlashProposal account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }

        let expected_pubkey =
            Self::find_program_address(program_id, ncn.key, operator.key, resolver.key).0;
        if slash_proposal.key.ne(&expected_pubkey) {
            msg!("SlashProposal account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
