use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum ProposalStatus {
    Initialization,
    WaitingApproval,
    Approved,
    Declined,
    PartiallyExecuted,
    Executed,
    Finalized,
    Error,
}

#[account]
pub struct Proposal {
    pub organization: Pubkey,
    pub index: u64,
    pub initializer: Pubkey,
    pub initializer_refund: u64,
    pub instructions_amount: u64,
    pub executed_instructions: u64,
    pub status: ProposalStatus,
    pub approval: u64,
    pub disapproval: u64,
    pub threshold: u64,
    pub total: u64,
    pub submit_timestamp: i64,
    pub vote_end_timestamp: i64,
    pub execution_timestamp: i64,
    pub security: bool,
    uri: Option<String>,
}
impl Proposal {
    pub const URI_LEN: usize = 150;

    pub const SIZE: usize = 8 // key
    + 32 // organization
    + 8 // index
    + 32 // initializer
    + 8 // initializer_refund (lamport)
    + 8 // instructions_amount
    + 8 // executed_instructions
    + 1 // status
    + 8 // approval (token amount)
    + 8 // disapproval (token amount)
    + 8 // threshold (token amount)
    + 8 // total (token amount)
    + 8 // submit_timestamp
    + 8 // vote_end_timestamp
    + 8 // execution_timestamp
    + 1 // security
    + 1 + 4 + (4 * Proposal::URI_LEN); // uri

    pub fn uri(&self) -> Option<String> {
        self.uri.clone()
    }

    pub fn set_uri(&mut self, uri: Option<String>) -> Result<()> {
        match uri {
            Some(s) => {
                if s.len() > Proposal::URI_LEN {
                    return Err(ErrorCode::StringTooLong.into());
                }
                self.uri = Some(s);
            }
            None => self.uri = uri,
        }

        Ok(())
    }
}
