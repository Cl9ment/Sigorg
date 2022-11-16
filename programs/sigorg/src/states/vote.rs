use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

#[account]
pub struct Vote {
    pub voter: Pubkey,
    pub organization: Pubkey,
    pub proposal: Pubkey,
    pub vote_timestamp: i64,
    pub approval: bool,
    pub token_amount: u64,
    uri: Option<String>,
}
impl Vote {
    const URI_LEN: usize = 150;

    pub const SIZE: usize = 8 // key
    + 32 // voter
    + 32 // organization
    + 32 // proposal
    + 8 // vote_timestamp
    + 1 // approval
    + 8 // token_amount
    + 1 + 4 + (4 * Vote::URI_LEN); // note

    pub fn uri(&self) -> Option<String> {
        self.uri.clone()
    }

    pub fn set_uri(&mut self, uri: Option<String>) -> Result<()> {
        match uri {
            Some(n) => {
                if n.len() > Vote::URI_LEN {
                    return Err(ErrorCode::StringTooLong.into());
                }
                self.uri = Some(n);
            }
            None => self.uri = uri,
        }

        Ok(())
    }
}
