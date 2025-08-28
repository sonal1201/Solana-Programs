use anchor_lang::prelude::*;

pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
pub const MINT_ACCOUNT_SEED: &[u8] = b"mint";

pub const MINT_DECIMAL: u8 = 9;
pub const LIQUIDATION: u8 = 50;
pub const LIQUIDATION_BONUS: u8 = 10;
pub const MIN_HEALTH_FACTOR: u8 = 1;