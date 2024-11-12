use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata};
use anchor_spl::{token_2022::Token2022, token_interface::Mint};
use mpl_token_metadata::instructions::CreateV1CpiBuilder;
use mpl_token_metadata::types::{DataV2, TokenStandard};

#[derive(Accounts)]
pub struct CreateRewardToken<'info> {
    #[account(
        init,
        seeds = [REWARD_MINT_SEED],
        bump,
        payer = admin,
        mint::decimals = 9,
        mint::authority = admin,
        mint::freeze_authority = admin,
        mint::token_program = token_program,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), reward_mint.key().as_ref()],
        bump,
        seeds::program = metadata_program.key(),
    )]
    /// CHECK: Checked in the instruction
    pub reward_mint_metadata: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
    pub metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
    #[account(address = sysvar::instructions::id())]
    /// CHECK: Instruction ssysvar account
    pub sysvar_instructions: UncheckedAccount<'info>,
}

pub fn process_create_reward_token(ctx: Context<CreateRewardToken>) -> Result<()> {
    msg!("Reward token Address:{}", ctx.accounts.reward_mint.key());
    CreateV1CpiBuilder::new(&ctx.accounts.metadata_program.to_account_info())
        .metadata(&ctx.accounts.reward_mint_metadata.to_account_info())
        .mint(&ctx.accounts.reward_mint.to_account_info(), false)
        .authority(&ctx.accounts.admin.to_account_info())
        .payer(&ctx.accounts.admin.to_account_info())
        .update_authority(&ctx.accounts.admin.to_account_info(), true)
        .spl_token_program(Some(&ctx.accounts.token_program.to_account_info()))
        .system_program(&ctx.accounts.system_program.to_account_info())
        .sysvar_instructions(&ctx.accounts.sysvar_instructions.to_account_info())
        .token_standard(TokenStandard::Fungible)
        .name(String::from("Reward Token"))
        .symbol(String::from("RWT"))
        .seller_fee_basis_points(0)
        .is_mutable(true)
        .uri("".to_string())
        .invoke()?;

    msg!("Reward token Address:{}", ctx.accounts.reward_mint.key());
    Ok(())
}
