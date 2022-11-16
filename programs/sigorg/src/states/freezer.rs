use anchor_lang::prelude::*;

#[account]
pub struct Freezer {
    pub organization: Pubkey,
    pub token_mint: Pubkey,
    pub token_account: Pubkey,

    freeze_timestamp: i64,
    thaw_timestamp: i64,
}
impl Freezer {
    pub const SIZE: usize = 8 // key
    + 32 // organization
    + 32 // token_mint
    + 32 // token_account
    + 8 // freeze_timestamp
    + 8; // thaw_timestamp

    pub fn is_frozen(&self) -> bool {
        Clock::get().unwrap().unix_timestamp < self.thaw_timestamp
    }

    pub fn freeze_until(&mut self, thaw_timestamp: i64) -> Result<()> {
        if self.thaw_timestamp < thaw_timestamp {
            self.thaw_timestamp = thaw_timestamp;
            if !self.is_frozen() {
                self.freeze_timestamp = Clock::get().unwrap().unix_timestamp;
            }
        }

        Ok(())
    }
}
