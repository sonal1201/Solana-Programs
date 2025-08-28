use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Colateral {
    pub depostior: PubKey,
    pub sol_account: PubKey,
    pub token_account: PubKey,
    pub lamport_balance: u64,
    pub account_minted: u64,
    pub bump: u8,
    pub bump_sol_account: u8,
    pub is_initialize: bool,
}

#[account]
#[derive(InitSpace,Debug)]
pub struct Config {
    pub authority: Pubkey,
    pub mint_account: Pubkey,
    pub liquidation: u64,
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
    pub bump: u8,
    pub bump_mint_account: u8,
}