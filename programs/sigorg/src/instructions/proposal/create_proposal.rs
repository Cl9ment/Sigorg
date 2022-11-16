use crate::constants::{
    global::MEMBER_TOKENS_DECIMALS,
    seeds::{STATE_ORGANIZATION, STATE_PROPOSAL},
    fees::{COLLECTOR, CREATE_PROPOSAL},
};
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use solana_program::system_program::ID as SYSTEM_PROGRAM_ID;
use anchor_spl::token::Mint;
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
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
        mut,
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
        init,
        payer = initializer,
        space = Proposal::SIZE,
        seeds = [
            STATE_PROPOSAL,
            organization.key().as_ref(),
            organization.proposal_index.to_be_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Box<Account<'info, Proposal>>,

    /// CHECK: This is not dangerous because we don't read or write from this account.
    #[account(
        mut,
        address = COLLECTOR,
        owner = SYSTEM_PROGRAM_ID,
    )]
    pub fee_collector: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_proposal_handler(ctx: Context<CreateProposal>) -> Result<()> {

    // Proposal state
    let proposal = &mut ctx.accounts.proposal;
    proposal.organization = ctx.accounts.organization.key();
    proposal.index = ctx.accounts.organization.proposal_index;
    proposal.initializer = ctx.accounts.initializer.key();
    proposal.initializer_refund =
        CREATE_PROPOSAL + Rent::get()?.minimum_balance(Proposal::SIZE);
    proposal.instructions_amount = 0;
    proposal.executed_instructions = 0;
    proposal.status = ProposalStatus::Initialization;
    proposal.approval = 0;
    proposal.disapproval = 0;

    // Updated on submit
    proposal.threshold = 0;
    proposal.total = 0;
    proposal.submit_timestamp = 0;
    proposal.vote_end_timestamp = 0;
    proposal.execution_timestamp = 0;
    proposal.security = false;
    proposal.set_uri(None)?;

    // Organization state
    let organization = &mut ctx.accounts.organization;
    organization.proposal_index += 1;

    // Proposal creation fee
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.initializer.to_account_info(),
                to: ctx.accounts.fee_collector.to_account_info(),
            },
        ),
        CREATE_PROPOSAL,
    )?;

    Ok(())
}