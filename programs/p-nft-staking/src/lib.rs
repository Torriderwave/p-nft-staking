use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;

pub mod constant;
pub mod error;
pub mod instructions;
pub mod state;
use constant::*;
use error::*;
use instructions::*;
use state::*;

declare_id!("2ktXheqvpk3w5sMEFeacmDH2erRKsZwXmz2GFQZz3vGf");

#[program]
pub mod p_nft_staking {
    use super::*;

    pub fn lock_pnft(ctx: Context<LockPNFT>) -> Result<()> {
        lock_pnft::lock_pnft_handler(ctx)
    }


    pub fn unlock_pnft(ctx: Context<UnlockPNFT>) -> Result<()> {
        unlock_pnft::unlock_pnft_handler(ctx)
    }
}
