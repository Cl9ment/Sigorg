use crate::constants::seeds::STATE_METADATA;
use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitUserMetadata<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = Metadata::SIZE,
        seeds = [
            STATE_METADATA,
            user.key().as_ref(),
        ],
        bump,
    )]
    pub metadata: Box<Account<'info, Metadata>>,

    pub system_program: Program<'info, System>,
}

pub fn init_user_metadata_handler(
    ctx: Context<InitUserMetadata>,
    name: Option<String>,
    uri: Option<String>,
) -> Result<()> {
    // Metadata state
    let metadata = &mut ctx.accounts.metadata;
    metadata.target = ctx.accounts.user.key();
    metadata.target_type = MetadataTargetType::User;
    metadata.set_name(name)?;
    metadata.set_symbol(None)?;
    metadata.set_uri(uri)?;

    Ok(())
}
