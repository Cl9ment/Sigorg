use crate::constants::{
    global::TEAM_TOKENS_DECIMALS,
    seeds::{ORGANIZATION_VAULT, STATE_ORGANIZATION},
};
use crate::states::organization::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{freeze_account, mint_to, FreezeAccount, Mint, MintTo, Token, TokenAccount},
};
use solana_program::program_option::COption;

#[derive(Accounts)]
pub struct AddTeamMember<'info> {
    #[account(
        signer,
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
        signer,
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
        init_if_needed,
        payer = organization_vault,
        associated_token::mint = team_token_mint,
        associated_token::authority = receiver,
    )]
    pub receiver_team_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn add_team_member_handler(ctx: Context<AddTeamMember>) -> Result<()> {
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

    // Mint 1 member token to the initializer
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.team_token_mint.to_account_info(),
                to: ctx.accounts.receiver.to_account_info(),
                authority: ctx.accounts.organization.to_account_info(),
            },
            signature,
        ),
        1,
    )?;

    // Freeze
    freeze_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        FreezeAccount {
            account: ctx.accounts.receiver_team_account.to_account_info(),
            mint: ctx.accounts.team_token_mint.to_account_info(),
            authority: ctx.accounts.organization.to_account_info(),
        },
        signature,
    ))?;

    Ok(())
}
