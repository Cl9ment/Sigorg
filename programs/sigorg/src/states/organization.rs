use anchor_lang::prelude::*;

#[account]
pub struct Organization {
    pub nonce: u8,
    pub creation_timestamp: i64,
	pub initializer: Pubkey,
	pub native_vault: Pubkey,
	pub member_token_mint: Pubkey,
	pub team_token_mint: Pubkey,

    pub proposal_index: u64,
	pub protocol_index: u64,
    pub dividend_index: u64,
    
	// settings
	pub default_voting_time: i64,
	pub security_voting_time: i64,
	pub dividend_claiming_time: u64,
	pub required_token_to_submit_proposal: f64,
	pub vote_reward: f64,
}
impl Organization {
    pub const SIZE: usize = 8 // key
	+ 1 // nonce
    + 8 // creation_timestamp
	+ 32 // initializer
	+ 32 // native_vault
	+ 32 // member_token_mint
	+ 32 // team_token_mint

    + 8 // proposal_index
    + 8 // protocol_index
    + 8 // dividend_index
    
    + 8 // default_voting_time
    + 8 // security_voting_time
    + 8 // dividend_claiming_time
    + 8 // required_token_to_submit_proposal
	+ 8; // vote_reward
}