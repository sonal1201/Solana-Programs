use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};
declare_id!("CJguR37LAm4w7iJqro3KTKVfr7Wv1CmMnQiK29t7pfBu");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize_vault(ctx: Context<VaultInit>, username: String) -> Result<()> {
        let initialize_vault = &mut ctx.accounts.initialize_vault;
        initialize_vault.owner = *ctx.accounts.owner.key;
        initialize_vault.bump = ctx.bumps.initialize_vault;
        initialize_vault.balance = 0;
        initialize_vault.username = username;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let initialize_vault = &mut ctx.accounts.initialize_vault;
        let owner = &ctx.accounts.owner;

        initialize_vault.balance = initialize_vault
            .balance
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        let tx = system_instruction::transfer(
            &ctx.accounts.owner.key,
            &ctx.accounts.initialize_vault.key(),
            amount,
        );

        invoke(
            &tx,
            &[
                ctx.accounts.owner.to_account_info(),
                ctx.accounts.initialize_vault.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn withdrawal(ctx: Context<Withdrawal>, amount: u64) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);

        let initialize_vault = &mut ctx.accounts.initialize_vault;
        require!(
            initialize_vault.balance >= amount,
            VaultError::InsufficientBalance
        );
        initialize_vault.balance = initialize_vault
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::Overflow)?;

        Ok(())
    }

    pub fn change_username(ctx: Context<ChnageUsername>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct VaultInit<'info> {
    #[account{
        init,
        seeds = [b"vault", username.as_bytes(), owner.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + VaultAccount::INIT_SPACE
    }]
    pub initialize_vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [b"vault", username.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub initialize_vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(username: String)]

pub struct Withdrawal<'info> {
    #[account(
        mut,
        seeds = [b"vault", username.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub initialize_vault: Account<'info, VaultAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultAccount {
    pub owner: Pubkey,
    pub bump: u8,
    pub balance: u64,
    #[max_len(50)]
    pub username: String,
}

#[error_code]
pub enum VaultError {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Insufficient balance for withdrawal")]
    InsufficientBalance,
    #[msg("Arithmetic overflow")]
    Overflow,
}
