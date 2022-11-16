use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use crate::states::*;
use crate::constants::{
    global::MEMBER_TOKENS_DECIMALS,
    seeds::{STATE_ORGANIZATION, STATE_PROPOSAL},
};
use anchor_spl::token::Mint;
use solana_program::program_option::COption;
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(proposal_index: u64)]
pub struct SubmitProposal<'info> {
    pub initializer: Signer<'info>,

    #[account(
        constraint = initializer_token_account.owner == initializer.key(),
        constraint = (
            initializer_token_account.mint == organization.team_token_mint && 
            initializer_token_account.amount >= 1 && 
            initializer_token_account.is_frozen()) || (
                initializer_token_account.mint == organization.member_token_mint && 
                initializer_token_account.amount >= (organization.required_token_to_submit_proposal * (member_token_mint.supply as f64)).ceil() as u64
            )
    )]
    pub initializer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            STATE_ORGANIZATION,
            organization.member_token_mint.as_ref(),
        ],
        bump,
    )]
    pub organization: Box<Account<'info, Organization>>,

    #[account(
        mut,
        constraint = member_token_mint.decimals == MEMBER_TOKENS_DECIMALS,
        constraint = member_token_mint.mint_authority == COption::Some(organization.key()),
        constraint = member_token_mint.freeze_authority == COption::Some(organization.key()),
        address = organization.member_token_mint,
    )]
    pub member_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [
            STATE_PROPOSAL,
            organization.key().as_ref(),
            proposal_index.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = organization,
        constraint = proposal.index == proposal_index,
        constraint = proposal.initializer == initializer.key() @ ErrorCode::NotAllowed,
        constraint = proposal.status == ProposalStatus::Initialization,
        constraint = proposal.instructions_amount > 0 @ ErrorCode::NoInstruction,
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn submit_proposal_handler(
    ctx: Context<SubmitProposal>,
    execution_timestamp: i64,
    security: bool,
    uri: Option<String>,
) -> Result<()> {
    
    // Proposal state
    let proposal = &mut ctx.accounts.proposal;
    let clock: Clock = Clock::get().unwrap();
    
    proposal.status = ProposalStatus::WaitingApproval;
    proposal.threshold = ((ctx.accounts.member_token_mint.supply as f64) * 0.5).floor() as u64;
    proposal.total = ctx.accounts.member_token_mint.supply;
    proposal.submit_timestamp = clock.unix_timestamp;
    proposal.vote_end_timestamp = match security {
        true => clock.unix_timestamp + ctx.accounts.organization.security_voting_time,
        false => clock.unix_timestamp + ctx.accounts.organization.default_voting_time,
    };
    proposal.execution_timestamp = execution_timestamp;
    proposal.security = security;
    proposal.set_uri(uri)?;

    Ok(())
}