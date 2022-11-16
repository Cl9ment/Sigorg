use crate::states::*;
use crate::errors::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use crate::constants::{
    global::MEMBER_TOKENS_DECIMALS,
    seeds::{STATE_ORGANIZATION, STATE_PROPOSAL, STATE_INSTRUCTION},
};
use anchor_spl::token::Mint;
use solana_program::program_option::COption;

#[derive(Accounts)]
#[instruction(proposal_index: u64)]
pub struct AddInstruction<'info> {
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
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(
        init,
        payer = initializer,
        space = Instruction::SIZE,
        seeds = [
            STATE_INSTRUCTION,
            proposal.key().as_ref(),
            proposal.instructions_amount.to_be_bytes().as_ref(),
        ],
        bump,
    )]
    pub instruction: Box<Account<'info, Instruction>>,

    pub system_program: Program<'info, System>,
}

pub fn add_instruction_handler(
    ctx: Context<AddInstruction>,
    instruction_program_id: Pubkey,
    instruction_accounts: Vec<InstructionAccount>,
    data: Vec<u8>,
) -> Result<()> {

    // Instruction state
    let instruction = &mut ctx.accounts.instruction;
    instruction.organization = ctx.accounts.proposal.organization;
    instruction.proposal = ctx.accounts.proposal.key();
    instruction.index = ctx.accounts.proposal.instructions_amount;
    instruction.did_execute = false;
    instruction.program_id = instruction_program_id;
    instruction.accounts = instruction_accounts;
    instruction.data = data;

    // Proposal state
    let proposal = &mut ctx.accounts.proposal;
    proposal.instructions_amount += 1;
    proposal.initializer_refund += Rent::get()?.minimum_balance(Instruction::SIZE);

    Ok(())
}