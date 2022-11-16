use crate::constants::{
    global::{MEMBER_TOKENS_DECIMALS, ORGANIZATION_VAULT_FUND, TEAM_TOKENS_DECIMALS},
    seeds::{ORGANIZATION_VAULT, STATE_ORGANIZATION},
};
use crate::states::organization::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct CreateOrganization<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
        init,
        payer = initializer,
        mint::decimals = MEMBER_TOKENS_DECIMALS,
        mint::authority = organization,
        mint::freeze_authority = organization,
    )]
    pub member_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = initializer,
        mint::decimals = TEAM_TOKENS_DECIMALS,
        mint::authority = organization,
        mint::freeze_authority = organization,
    )]
    pub team_token_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = initializer,
        space = Organization::SIZE,
        seeds = [
            STATE_ORGANIZATION,
            member_token_mint.key().as_ref(),
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
        init,
        payer = initializer,
        associated_token::mint = member_token_mint,
        associated_token::authority = initializer,
    )]
    pub initializer_member_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn create_organization_handler(ctx: Context<CreateOrganization>) -> Result<()> {
    // Organization state
    let organization = &mut ctx.accounts.organization;
    organization.nonce = *ctx.bumps.get("organization").unwrap();
    organization.creation_timestamp = Clock::get()?.unix_timestamp;
    organization.initializer = ctx.accounts.initializer.key();
    organization.native_vault = ctx.accounts.organization_vault.key();
    organization.member_token_mint = ctx.accounts.member_token_mint.key();
    organization.team_token_mint = ctx.accounts.team_token_mint.key();
    organization.proposal_index = 0;
    organization.protocol_index = 0;
    organization.dividend_index = 0;
    organization.default_voting_time = 60 * 60 * 24 * 7; // 7 days
    organization.security_voting_time = 60 * 60 * 24 * 1; // 1 days
    organization.dividend_claiming_time = 60 * 60 * 24 * 7; // 7 days
    organization.required_token_to_submit_proposal = 0.01; // 1 %
    organization.vote_reward = 0.005; // 0.5 %

    // Fund organization vault
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.initializer.to_account_info(),
                to: ctx.accounts.organization_vault.to_account_info(),
            },
        ),
        ORGANIZATION_VAULT_FUND,
    )?;

    // Signature
    let member_token_mint = organization.member_token_mint.clone();
    let seed_bump = organization.nonce.to_le_bytes();
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
                mint: ctx.accounts.member_token_mint.to_account_info(),
                to: ctx.accounts.initializer_member_account.to_account_info(),
                authority: ctx.accounts.organization.to_account_info(),
            },
            signature,
        ),
        10u64.pow(MEMBER_TOKENS_DECIMALS as u32),
    )?;

    Ok(())
}
