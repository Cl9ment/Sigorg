use crate::errors::ErrorCode;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq)]
pub enum MetadataTargetType {
    Organization,
    User,
    Proposal,
    Program,
}

#[account]
pub struct Metadata {
    pub target: Pubkey,
    pub target_type: MetadataTargetType,
    name: Option<String>,
    symbol: Option<String>,
    uri: Option<String>,
    last_change: i64,
}
impl Metadata {
    pub const NAME_LEN: usize = 100;
    pub const SYMBOL_LEN: usize = 6;
    pub const URI_LEN: usize = 150;

    pub const SIZE: usize = 8 // key
    + 32 // target
    + 1 // target_type
    + 1 + 4 + (4 * Metadata::NAME_LEN) // name
    + 1 + 4 + (4 * Metadata::SYMBOL_LEN) // symbol
    + 1 + 4 + (4 * Metadata::URI_LEN) // uri
    + 8; // last_change

    pub fn last_change(&self) -> i64 {
        self.last_change.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: Option<String>) -> Result<()> {
        match name {
            Some(n) => {
                if n.len() > Metadata::NAME_LEN {
                    return Err(ErrorCode::StringTooLong.into());
                }
                self.name = Some(n);
            }
            None => self.name = name,
        }

        self.last_change = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    pub fn symbol(&self) -> Option<String> {
        self.symbol.clone()
    }

    pub fn set_symbol(&mut self, symbol: Option<String>) -> Result<()> {
        match symbol {
            Some(n) => {
                if n.len() > Metadata::SYMBOL_LEN {
                    return Err(ErrorCode::StringTooLong.into());
                }
                self.symbol = Some(n);
            }
            None => self.symbol = symbol,
        }

        self.last_change = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    pub fn uri(&self) -> Option<String> {
        self.uri.clone()
    }

    pub fn set_uri(&mut self, uri: Option<String>) -> Result<()> {
        match uri {
            Some(n) => {
                if n.len() > Metadata::URI_LEN {
                    return Err(ErrorCode::StringTooLong.into());
                }
                self.uri = Some(n);
            }
            None => self.uri = uri,
        }

        self.last_change = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }
}
