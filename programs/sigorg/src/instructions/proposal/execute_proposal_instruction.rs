use crate::constants::seeds::{
    ORGANIZATION_VAULT, STATE_INSTRUCTION, STATE_ORGANIZATION, STATE_PROPOSAL,
};
use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction as SolanaInstruction;
use anchor_lang::system_program::{transfer, Transfer};
use std::ops::Deref;

#[derive(Accounts)]
#[instruction(proposal_index: u64)]
pub struct ExecuteProposalInstruction<'info> {
    /// CHECK: This is not dangerous because we don't read or write from this account.
    #[account(
        mut,
        owner = anchor_lang::system_program::ID,
        address = proposal.initializer,
    )]
    pub initializer: AccountInfo<'info>,

    #[account(
        seeds = [
            STATE_ORGANIZATION,
            organization.member_token_mint.as_ref(),
        ],
        bump,
    )]
    pub organization: Box<Account<'info, Organization>>,

    /// CHECK: This is not dangerous because we don't read or write from this account.
    #[account(
        mut,
        owner = anchor_lang::system_program::ID,
        seeds = [
            ORGANIZATION_VAULT,
            organization.key().as_ref(),
        ],
        bump,
    )]
    pub organization_vault: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            STATE_PROPOSAL,
            organization.key().as_ref(),
            proposal_index.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = organization,
        constraint = proposal.status == ProposalStatus::Approved || proposal.status == ProposalStatus::PartiallyExecuted @ ErrorCode::NotAllowed,
        constraint = proposal.execution_timestamp <= Clock::get().unwrap().unix_timestamp @ ErrorCode::CannotExecuteNow,
    )]
    pub proposal: Box<Account<'info, Proposal>>,

    #[account(
        mut,
        seeds = [
            STATE_INSTRUCTION,
            proposal.key().as_ref(),
            proposal.executed_instructions.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = organization,
        has_one = proposal,
        constraint = instruction.did_execute == false,
        constraint = instruction.index == proposal.executed_instructions,
    )]
    pub instruction: Account<'info, Instruction>,

    pub system_program: Program<'info, System>,
}

pub fn execute_proposal_instruction_handler(
    ctx: Context<ExecuteProposalInstruction>,
) -> Result<()> {
    let instruction_copy = ctx.accounts.instruction.clone();
    let mut instruction: SolanaInstruction = instruction_copy.deref().into();

    // Accounts
    instruction.accounts = instruction
        .accounts
        .iter()
        .map(|acc| {
            let mut acc = acc.clone();
            if &acc.pubkey == &ctx.accounts.organization.key() {
                acc.is_signer = true;
            }
            acc
        })
        .collect();

    let proposal = &mut ctx.accounts.proposal;
    let instruction_acc = &mut ctx.accounts.instruction;

    // Signer seed
    let pda_1_seed = ctx.accounts.organization.member_token_mint.clone();
    let pda_1_bump = ctx.accounts.organization.nonce.to_le_bytes();
    let pda_seeds_1 = vec![
        STATE_ORGANIZATION.as_ref(),
        pda_1_seed.as_ref(),
        pda_1_bump.as_ref(),
    ];
    let pda_2_seed = ctx.accounts.organization.key().clone();
    let pda_2_bump_raw = *ctx.bumps.get("organization_vault").unwrap();
    let pda_2_bump = pda_2_bump_raw.to_le_bytes();
    let pda_seeds_2 = vec![
        ORGANIZATION_VAULT.as_ref(),
        pda_2_seed.as_ref(),
        pda_2_bump.as_ref(),
    ];
    let seeds = vec![pda_seeds_1.as_slice(), pda_seeds_2.as_slice()];
    let signer = seeds.as_slice();

    // Execute tx
    match solana_program::program::invoke_signed(&instruction, ctx.remaining_accounts, signer) {
        Ok(()) => {
            instruction_acc.did_execute = true;
            proposal.executed_instructions += 1;
            proposal.status = ProposalStatus::PartiallyExecuted;
        }
        Err(e) => {
            proposal.status = ProposalStatus::Error;
            msg!("Execution error: {}", e.to_string());
        }
    }

    // Refund & finalize proposal on last instruction
    if instruction_acc.index == (proposal.instructions_amount - 1) {
        // Signer seed (organization_vault).
        let organization_key = ctx.accounts.organization.key().clone();
        let seed_bump_ov = *ctx.bumps.get("organization_vault").unwrap();
        let seed_bump_ovb = seed_bump_ov.to_le_bytes();
        let inner = vec![
            ORGANIZATION_VAULT.as_ref(),
            organization_key.as_ref(),
            seed_bump_ovb.as_ref(),
        ];
        let outer_ov = vec![inner.as_slice()];
        let signature_ov = outer_ov.as_slice();

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.organization_vault.to_account_info(),
                    to: ctx.accounts.initializer.to_account_info(),
                },
                signature_ov,
            ),
            proposal.initializer_refund,
        )?;

        proposal.status = ProposalStatus::Finalized;
    }

    Ok(())
}
