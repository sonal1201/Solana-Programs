use anchor_lang::prelude::*;

declare_id!("2jnNCZhLaZ3ntiuWb8ib8M1kd5vHdhKCF2DJVU9mmg2H");

#[program]
pub mod program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
