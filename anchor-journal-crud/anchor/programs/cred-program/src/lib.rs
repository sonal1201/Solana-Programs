use anchor_lang::prelude::*;

declare_id!("FqzkXZdwYjurnUKetJCAvaUw5WAqbwzU6gZEwydeEfqS");

#[program]
pub mod counter {
    use super::*;

    pub fn create_journal(
        ctx: Context<JournalEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn update_journal(
        ctx: Context<JournalUpdate>,
        _title: String,
        message: String,
    ) -> Result<()> {
        let jornal_entry = &mut ctx.accounts.journal_entry;
        jornal_entry.message = message;

        Ok(())
    }

    pub fn delete_journal(_ctx: Context<JournalDelete>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct JournalEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + JournalState::INIT_SPACE,

    )]
    pub journal_entry: Account<'info, JournalState>,
    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct JournalUpdate<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + JournalState::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true
    )]
    pub journal_entry: Account<'info, JournalState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct JournalDelete<'info> {
    #[account{
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    }]
    pub journal_entry: Account<'info, JournalState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct JournalState {
    pub owner: Pubkey,
    #[max_len(31)]
    pub title: String,
    #[max_len(500)]
    pub message: String,
}
