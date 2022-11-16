use anchor_lang::prelude::*;
use anchor_lang::solana_program::instruction::Instruction as SolanaInstruction;
use std::convert::Into;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct InstructionAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
impl From<&InstructionAccount> for AccountMeta {
    fn from(account: &InstructionAccount) -> AccountMeta {
        match account.is_writable {
            false => AccountMeta::new_readonly(account.pubkey, account.is_signer),
            true => AccountMeta::new(account.pubkey, account.is_signer),
        }
    }
}

#[account()]
pub struct Instruction {
    pub organization: Pubkey,
    pub proposal: Pubkey,
    pub index: u64,
    pub did_execute: bool,
    pub program_id: Pubkey,
    pub accounts: Vec<InstructionAccount>,
    pub data: Vec<u8>,
}
impl Instruction {
    pub const SIZE: usize = 1232;
}
impl From<&Instruction> for SolanaInstruction {
    fn from(instruction: &Instruction) -> SolanaInstruction {
        SolanaInstruction {
            program_id: instruction.program_id,
            accounts: instruction.accounts.iter().map(Into::into).collect(),
            data: instruction.data.clone(),
        }
    }
}
