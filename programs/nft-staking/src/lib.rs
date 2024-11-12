pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("J2x7MHeVj9HY2oooJrR2Yz7Ne5bmpVvd3gt2vSoAZ1MN");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn create_reward_token(ctx: Context<CreateRewardToken>) -> Result<()> {
        process_create_reward_token(ctx)
    }
}
