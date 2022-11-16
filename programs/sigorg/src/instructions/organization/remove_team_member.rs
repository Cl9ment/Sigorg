use crate::constants::{global::TEAM_TOKENS_DECIMALS, seeds::STATE_ORGANIZATION};
use crate::states::organization::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{thaw_account, Mint, ThawAccount, Token, TokenAccount};
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct RemoveTeamMember<'info> {
    #[account(
        signer,
        seeds = [
            STATE_ORGANIZATION,
            organization.member_token_mint.as_ref(),
        ],
        bump,
    )]
    pub organization: Box<Account<'info, Organization>>,

    #[account(
        constraint = team_token_mint.decimals == TEAM_TOKENS_DECIMALS,
        constraint = team_token_mint.mint_authority == COption::Some(organization.key()),
        constraint = team_token_mint.freeze_authority == COption::Some(organization.key()),
        address = organization.team_token_mint,
    )]
    pub team_token_mint: Box<Account<'info, Mint>>,

    /// CHECK: This is not dangerous because we don't read or write from this account.
    #[account(owner = anchor_lang::system_program::ID)]
    pub receiver: AccountInfo<'info>,

    #[account(
        constraint = receiver_team_account.owner == receiver.key(),
        constraint = receiver_team_account.mint == team_token_mint.key(),
        constraint = receiver_team_account.amount >= 1,
        constraint = receiver_team_account.is_frozen(),
    )]
    pub receiver_team_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

pub fn remove_team_member_handler(ctx: Context<RemoveTeamMember>) -> Result<()> {
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

    // Thaw
    thaw_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        ThawAccount {
            account: ctx.accounts.receiver_team_account.to_account_info(),
            mint: ctx.accounts.team_token_mint.to_account_info(),
            authority: ctx.accounts.organization.to_account_info(),
        },
        signature,
    ))?;

    Ok(())
}
