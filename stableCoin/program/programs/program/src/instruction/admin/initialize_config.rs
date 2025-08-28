use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint,Token2022};
use create::{Config, MINT_ACCOUNT_SEED, SEED_CONFIG_ACCOUNT, MINT_DECIMAL};

use crate::{constant::{LIQUIDATION, LIQUIDATION_BONUS, MIN_HEALTH_FACTOR}, state::Config};

pub fn process_initialize_config(ctx:Context<InitializeConfig>) -> Result<()> {
    *ctx.accounts.config_account = Config {
        authority: ctx.accounts.signer.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation: LIQUIDATION,
        liquidation_bonus: LIQUIDATION_BONUS,
        min_health_factor: MIN_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.accounts.mint_account
    }
    Ok(());
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = signer,
        seeds = [MINT_ACCOUNT_SEED],
        bump,
        mint::decimals = MINT_DECIMAL,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program 
    )]

    pub mint_account: InterfaceAccount<'info,Mint>,
    pub token_program: Program<'info,Token2022>,

    pub system_program: Program<'info, System>,
}
