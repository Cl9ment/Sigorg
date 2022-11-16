use crate::constants::{
    global::MEMBER_TOKENS_DECIMALS,
    seeds::{STATE_FREEZER, STATE_ORGANIZATION},
};
use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{thaw_account, Mint, ThawAccount, Token, TokenAccount};
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct ThawMemberTokenAccount<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        mut,
        constraint = member_token_account.owner == member.key(),
        constraint = member_token_account.mint == organization.member_token_mint,
        constraint = member_token_account.is_frozen(),
    )]
    pub member_token_account: Box<Account<'info, TokenAccount>>,

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
        seeds = [
            STATE_FREEZER,
            member_token_account.key().as_ref(),
        ],
        bump,
        constraint = !freezer.is_frozen() @ ErrorCode::AccountIsFrozen,
    )]
    pub freezer: Box<Account<'info, Freezer>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

pub fn thaw_member_token_account_handler(ctx: Context<ThawMemberTokenAccount>) -> Result<()> {
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
            account: ctx.accounts.member_token_account.to_account_info(),
            mint: ctx.accounts.member_token_mint.to_account_info(),
            authority: ctx.accounts.organization.to_account_info(),
        },
        signature,
    ))?;

    Ok(())
}
