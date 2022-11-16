use crate::constants::seeds::STATE_METADATA;
use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EditUserMetadata<'info> {
    #[account()]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [
            STATE_METADATA,
            user.key().as_ref(),
        ],
        bump,
        constraint = metadata.target == user.key(),
        constraint = metadata.target_type == MetadataTargetType::User,
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    pub system_program: Program<'info, System>,
}

pub fn edit_user_metadata_handler(
    ctx: Context<EditUserMetadata>,
    name: Option<String>,
    uri: Option<String>,
) -> Result<()> {
    // Metadata state
    let metadata = &mut ctx.accounts.metadata;
    metadata.set_name(name)?;
    metadata.set_uri(uri)?;

    Ok(())
}
