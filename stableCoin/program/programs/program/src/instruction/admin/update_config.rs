use anchor_lang::prelude::*;

use crate::{Config, SEED_CONFIG_ACCOUNT};

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
    )]

    pub config_accouct: Account<'info,Config>
}

pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
   let config_account = &mut ctx.accounts.config_accouct;

   config_account.min_health_factor = min_health_factor;
}