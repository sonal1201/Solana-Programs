use anchor_lang::prelude::*;

use instruction::*;
mod instruction;

use state::*;
mod state;

use constant::*;
mod constant;

declare_id!("GrxqShgMkapmRJ3dFG9tG2A6BnyxQ3JfLLWZ7oarcEqq");

#[program]
pub mod program {
    use super::*;

    pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx);
    }

    pub fn update_config(ctx: Context<UpdateConfig>)-> Result<()> {
        update_config(ctx);
    }
}
