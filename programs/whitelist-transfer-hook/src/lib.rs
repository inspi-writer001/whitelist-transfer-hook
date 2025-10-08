#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

mod errors;
mod instructions;
mod state;
use instructions::*;

use spl_discriminator::SplDiscriminate;
use spl_tlv_account_resolution::state::ExtraAccountMetaList;
use spl_transfer_hook_interface::instruction::{
    ExecuteInstruction, InitializeExtraAccountMetaListInstruction,
};

declare_id!("CVWZc8DyWjzJWJaE1ro8tGr4i1DHNVRiUufQJ4nGPfzS");

#[program]
pub mod whitelist_transfer_hook {
    use super::*;

    // pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>) -> Result<()> {
    //     ctx.accounts.initialize_whitelist(ctx.bumps)
    // }

    pub fn add_to_whitelist(
        ctx: Context<WhitelistOperations>,
        user_source_token: Pubkey,
    ) -> Result<()> {
        ctx.accounts.add_to_whitelist(user_source_token, &ctx.bumps)
    }

    pub fn remove_from_whitelist(
        ctx: Context<WhitelistOperations>,
        user_source_token: Pubkey,
    ) -> Result<()> {
        ctx.accounts.remove_from_whitelist()
    }

    #[instruction(discriminator = InitializeExtraAccountMetaListInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn initialize_transfer_hook(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
        msg!("Initializing Transfer Hook...");

        // Get the extra account metas for the transfer hook
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        msg!("Extra Account Metas: {:?}", extra_account_metas);
        msg!("Extra Account Metas Length: {}", extra_account_metas.len());

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas,
        )?;

        Ok(())
    }

    #[instruction(discriminator = ExecuteInstruction::SPL_DISCRIMINATOR_SLICE)]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        // Call the transfer hook logic
        ctx.accounts.transfer_hook(amount)
    }
}
