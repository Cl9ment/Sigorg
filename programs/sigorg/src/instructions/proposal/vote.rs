use crate::constants::{
    global::MEMBER_TOKENS_DECIMALS,
    seeds::{STATE_FREEZER, STATE_ORGANIZATION, STATE_PROPOSAL, STATE_VOTE},
};
use crate::errors::ErrorCode;
use crate::states::*;
use crate::utils::is_proposal_approved;
use anchor_lang::prelude::*;
use anchor_spl::token::{
    freeze_account, mint_to, thaw_account, FreezeAccount, Mint, MintTo, ThawAccount, Token,
    TokenAccount,
};
use solana_program::program_option::COption;

#[derive(Accounts)]
#[instruction(proposal_index: u64)]
pub struct ProposalVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        mut,
        constraint = voter_member_token_account.owner == voter.key(),
        constraint = voter_member_token_account.mint == organization.member_token_mint,
    )]
    pub voter_member_token_account: Box<Account<'info, TokenAccount>>,

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
        constraint = proposal.status == ProposalStatus::WaitingApproval,
        constraint = proposal.instructions_amount > 0 @ ErrorCode::NoInstruction,
        constraint = proposal.vote_end_timestamp > Clock::get()?.unix_timestamp @ ErrorCode::VoteOver,
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = Vote::SIZE,
        seeds = [
            STATE_VOTE,
            proposal.key().as_ref(),
            voter_member_token_account.key().as_ref(),
        ],
        bump,
    )]
    pub vote: Box<Account<'info, Vote>>,

    #[account(
        mut,
        seeds = [
            STATE_FREEZER,
            voter_member_token_account.key().as_ref(),
        ],
        bump,
        constraint = freezer.organization == organization.key(),
        constraint = freezer.token_mint == member_token_mint.key(),
        constraint = freezer.token_account == voter_member_token_account.key(),
    )]
    pub freezer: Box<Account<'info, Freezer>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

pub fn vote_handler(ctx: Context<ProposalVote>, approval: bool, uri: Option<String>) -> Result<()> {
    // Vote state
    let vote = &mut ctx.accounts.vote;
    vote.voter = ctx.accounts.voter.key();
    vote.organization = ctx.accounts.organization.key();
    vote.proposal = ctx.accounts.proposal.key();
    vote.vote_timestamp = Clock::get()?.unix_timestamp;
    vote.approval = approval;
    vote.token_amount = ctx.accounts.voter_member_token_account.amount;
    vote.set_uri(uri)?;

    // Proposal state
    let proposal = &mut ctx.accounts.proposal;
    match approval {
        true => proposal.approval += vote.token_amount,
        false => proposal.disapproval += vote.token_amount,
    }

    // Freeze state
    let freezer = &mut ctx.accounts.freezer;
    freezer.freeze_until(proposal.vote_end_timestamp)?;

    // Signature
    let member_token_mint = ctx.accounts.organization.member_token_mint.clone();
    let seed_bump = ctx.accounts.organization.nonce.to_le_bytes();
    let inner = vec![
        STATE_ORGANIZATION.as_ref(),
        member_token_mint.as_ref(),
        seed_bump.as_ref(),
    ];
    let outer = vec![inner.as_slice()];
    let signature = outer.as_slice();

    // Reward voter
    if ctx.accounts.organization.vote_reward > 0_f64 {
        let reward_amount: u64 = ((ctx.accounts.voter_member_token_account.amount as f64)
            * ctx.accounts.organization.vote_reward)
            .floor() as u64;

        if reward_amount > 0 {
            if ctx.accounts.voter_member_token_account.is_frozen() {
                thaw_account(CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    ThawAccount {
                        account: ctx.accounts.voter_member_token_account.to_account_info(),
                        mint: ctx.accounts.member_token_mint.to_account_info(),
                        authority: ctx.accounts.organization.to_account_info(),
                    },
                    signature,
                ))?;
            }

            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.member_token_mint.to_account_info(),
                        to: ctx.accounts.voter_member_token_account.to_account_info(),
                        authority: ctx.accounts.organization.to_account_info(),
                    },
                    signature,
                ),
                reward_amount,
            )?;
        }
    }

    // Freeze
    if !ctx.accounts.voter_member_token_account.is_frozen() {
        freeze_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            FreezeAccount {
                account: ctx.accounts.voter_member_token_account.to_account_info(),
                mint: ctx.accounts.member_token_mint.to_account_info(),
                authority: ctx.accounts.organization.to_account_info(),
            },
            signature,
        ))?;
    }

    // Check if the proposal is approved
    let _approved = is_proposal_approved(proposal);

    Ok(())
}
