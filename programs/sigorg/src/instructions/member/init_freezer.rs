use crate::constants::seeds::{STATE_FREEZER, STATE_ORGANIZATION};
use crate::states::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct InitFreezer<'info> {
    #[account(mut)]
    pub member: Signer<'info>,

    #[account(
        constraint = member_token_account.owner == member.key(),
        constraint = member_token_account.mint == organization.member_token_mint,
        constraint = !member_token_account.is_frozen(),
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
        init,
        payer = member,
        space = Freezer::SIZE,
        seeds = [
            STATE_FREEZER,
            member_token_account.key().as_ref(),
        ],
        bump,
    )]
    pub freezer: Box<Account<'info, Freezer>>,

    pub system_program: Program<'info, System>,
}

pub fn init_freezer_handler(ctx: Context<InitFreezer>) -> Result<()> {
    // Freeze state
    let freezer = &mut ctx.accounts.freezer;
    freezer.organization = ctx.accounts.organization.key();
    freezer.token_mint = ctx.accounts.member_token_account.mint;
    freezer.token_account = ctx.accounts.member_token_account.key();
    freezer.freeze_until(0)?;

    Ok(())
}
