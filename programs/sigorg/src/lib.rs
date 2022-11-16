use anchor_lang::prelude::*;

pub mod constants;
pub use constants::*;

pub mod instructions;
pub use instructions::*;

pub mod states;
pub use states::*;

pub mod utils;
pub use utils::*;

pub mod errors;
pub use errors::*;

declare_id!("83X8rd5T7NDFiqHWGVgCLDx1jLMjEEg9JqxWQm52dpEK");

#[program]
pub mod sigorg {
    use super::*;

    // *** Member ***
    pub fn edit_user_metadata(
        ctx: Context<EditUserMetadata>,
        name: Option<String>,
        uri: Option<String>,
    ) -> Result<()> {
        edit_user_metadata_handler(ctx, name, uri)
    }

    pub fn init_freezer(ctx: Context<InitFreezer>) -> Result<()> {
        init_freezer_handler(ctx)
    }

    pub fn init_user_metadata(
        ctx: Context<InitUserMetadata>,
        name: Option<String>,
        uri: Option<String>,
    ) -> Result<()> {
        init_user_metadata_handler(ctx, name, uri)
    }

    pub fn thaw_member_token_account(ctx: Context<ThawMemberTokenAccount>) -> Result<()> {
        thaw_member_token_account_handler(ctx)
    }

    // *** Organization ***

    pub fn add_team_member(ctx: Context<AddTeamMember>) -> Result<()> {
        add_team_member_handler(ctx)
    }

    pub fn create_organization(ctx: Context<CreateOrganization>) -> Result<()> {
        create_organization_handler(ctx)
    }

    pub fn edit_organization_settings(
        ctx: Context<EditOrganizationSettings>,
        default_voting_time: i64,
        security_voting_time: i64,
        dividend_claiming_time: u64,
        required_token_to_submit_proposal: f64,
        vote_reward: f64,
    ) -> Result<()> {
        edit_organization_settings_handler(
            ctx,
            default_voting_time,
            security_voting_time,
            dividend_claiming_time,
            required_token_to_submit_proposal,
            vote_reward,
        )
    }

    pub fn remove_team_member(ctx: Context<RemoveTeamMember>) -> Result<()> {
        remove_team_member_handler(ctx)
    }

    // *** Proposal ***

    pub fn add_instruction(
        ctx: Context<AddInstruction>,
        _proposal_index: u64,
        instruction_program_id: Pubkey,
        instruction_accounts: Vec<InstructionAccount>,
        data: Vec<u8>,
    ) -> Result<()> {
        add_instruction_handler(ctx, instruction_program_id, instruction_accounts, data)
    }

    pub fn create_proposal(ctx: Context<CreateProposal>) -> Result<()> {
        create_proposal_handler(ctx)
    }

    pub fn execute_proposal_instruction(
        ctx: Context<ExecuteProposalInstruction>,
        _proposal_index: u64,
    ) -> Result<()> {
        execute_proposal_instruction_handler(ctx)
    }

    pub fn submit_proposal(
        ctx: Context<SubmitProposal>,
        _proposal_index: u64,
        execution_timestamp: i64,
        security: bool,
        uri: Option<String>,
    ) -> Result<()> {
        submit_proposal_handler(ctx, execution_timestamp, security, uri)
    }

    pub fn vote(
        ctx: Context<ProposalVote>,
        _proposal_index: u64,
        approval: bool,
        uri: Option<String>,
    ) -> Result<()> {
        vote_handler(ctx, approval, uri)
    }
}
