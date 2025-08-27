use anchor_lang::prelude::*;

#[account]
pub struct EscrowAccount {
    pub buyer: Pubkey,
    pub seller: Pubkey,
    pub arbiter: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub token_mint: Pubkey,
    pub expiry: i64,
    pub state: EscrowState,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum EscrowState {
    Created,
    Released,
    Refunded,
    Disputed,
    Resolved,
}
