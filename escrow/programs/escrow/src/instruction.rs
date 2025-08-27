use std::u64;

use crate::error::*;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::config::program;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64, expiry: i64) -> Result<()> {
    ///Logic
    let escrow = &mut ctx.accounts.escrow_account;

    escrow.buyer = ctx.accounts.buyer.key();
    escrow.seller = ctx.accounts.seller.key();
    escrow.arbiter = ctx.accounts.arbiter.key();
    escrow.mint = ctx.accounts.mint.key();
    escrow.amount = amount;
    escrow.bump = ctx.bumps.escrow_account;
    escrow.expiry = expiry;

    Ok(())
}

pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
    

    Ok(())
}

pub fn release(ctx: Context<Release>) -> Result<()> {
    ///Logic
    Ok(())
}

pub fn refund(ctx: Context<Refund>) -> Result<()> {
    ///Logic
    Ok(())
}

pub fn dispute(ctx: Context<Dispute>, winner: Pubkey) -> Result<()> {
    ///Logic
    Ok(())
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    pub seller: UncheckedAccount<'info>,

    pub arbiter: UncheckedAccount<'info>,
    #[account(
        init,
        payer = buyer,
        space = 8 + 160,
        seeds = [b"escrow",buyer.key().as_ref(),seller.key().as_ref()],
        bump

    )]
    pub escrow_account: Account<'info, EscrowAccount>,
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Dispute<'info> {
    #[account(mut)]
    pub arbiter: Signer<'info>,
}
