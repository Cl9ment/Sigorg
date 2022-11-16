use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("An argument is invalid.")]
    InvalidArgument,

    #[msg("The provided string is too long.")]
    StringTooLong,

    #[msg("You don't have the authorization to do that.")]
    NotAllowed,

    #[msg("The proposal must contain at least one instruction.")]
    NoInstruction,

    #[msg("You can't vote anymore.")]
    VoteOver,

    #[msg("You can't execute the proposal now.")]
    CannotExecuteNow,

    #[msg("Your token account is currently frozen.")]
    AccountIsFrozen,
}
