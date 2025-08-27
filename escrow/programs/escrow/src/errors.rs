use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Escrow has already been funded.")]
    AlreadyFunded,
    #[msg("Escrow expired.")]
    Expired,
    #[msg("Unauthorized action.")]
    Unauthorized,
    #[msg("Invalid state for this operation.")]
    InvalidState,
}
