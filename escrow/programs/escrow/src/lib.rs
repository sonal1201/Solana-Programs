use anchor_lang::prelude::*;

pub mod errors;
pub mod instruction;
pub mod state;

use instruction::*;

declare_id!("HE7ZAZuKvZsYo7PmxdGXTj9fbXW4LQf7eRUCH4qXEiap");

#[program]
pub mod escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64, expiry: i64) -> Result<()> {
        instruction::create_escrow(ctx, amount, expiry);
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instruction::deposit(ctx)
    }

    pub fn release(ctx: Context<Release>) -> Result<()> {
        instruction::release(ctx)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instruction::refund(ctx)
    }

    pub fn dispute(ctx: Context<Dispute>, winner: Pubkey) -> Result<()> {
        instruction::dispute(ctx, winner)
    }
}
