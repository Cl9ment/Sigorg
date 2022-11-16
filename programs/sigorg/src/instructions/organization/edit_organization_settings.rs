use crate::constants::seeds::STATE_ORGANIZATION;
use crate::states::organization::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditOrganizationSettings<'info> {
    #[account(
        signer,
        mut,
        seeds = [
            STATE_ORGANIZATION,
            organization.member_token_mint.as_ref(),
        ],
        bump,
    )]
    pub organization: Box<Account<'info, Organization>>,
}

pub fn edit_organization_settings_handler(
    ctx: Context<EditOrganizationSettings>,
    default_voting_time: i64,
    security_voting_time: i64,
    dividend_claiming_time: u64,
    required_token_to_submit_proposal: f64,
    vote_reward: f64,
) -> Result<()> {
    // Organization state
    let organization = &mut ctx.accounts.organization;
    organization.default_voting_time = default_voting_time;
    organization.security_voting_time = security_voting_time;
    organization.dividend_claiming_time = dividend_claiming_time;
    organization.required_token_to_submit_proposal = required_token_to_submit_proposal;
    organization.vote_reward = vote_reward;

    Ok(())
}
