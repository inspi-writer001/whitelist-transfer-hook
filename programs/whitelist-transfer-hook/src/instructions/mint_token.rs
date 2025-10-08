use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{
        self, mint_to_checked,
        spl_token_2022::{extension::ExtensionType, state::Mint as MintState},
        MintToChecked,
    },
    token_interface::{
        initialize_mint2, transfer_hook_initialize, InitializeMint2, Mint, TokenAccount,
        TokenInterface, TransferHookInitialize,
    },
};

use crate::state::Whitelist;

#[derive(Accounts)]
pub struct TokenFactory<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        mint::decimals = 9,
        mint::authority = user,
        extensions::transfer_hook::authority = user,
        extensions::transfer_hook::program_id = crate::ID,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub source_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: ExtraAccountMetaList Account, will be checked by the transfer hook
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [b"whitelist", source_token_account.key().as_ref()], 
        bump
    )]
    pub blocklist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TokenFactory<'info> {
    pub fn init_mint(&mut self, bumps: &TokenFactoryBumps) -> Result<()> {
        // create_account

        // let create_account_accounts = CreateAccount {
        //     from: self.user.to_account_info(),
        //     to: self.mint.to_account_info(),
        // };

        // let account_context = CpiContext::new(
        //     self.system_program.to_account_info(),
        //     create_account_accounts,
        // );

        // let extensions = vec![ExtensionType::TransferHook];
        // let space = ExtensionType::try_calculate_account_len::<MintState>(&extensions)?;

        // let lamports_needed = Rent::get()?.minimum_balance(space);

        // create_account(
        //     account_context,
        //     lamports_needed,
        //     space as u64,
        //     &self.token_program.key(),
        // )?;

        // msg!("Mint account created: {:#}", &self.mint.key());

        // let initialize_transfer_hook_ix = TransferHookInitialize {
        //     token_program_id: self.token_program.to_account_info(),
        //     mint: self.mint.to_account_info(),
        // };

        // let transfer_hook_context = CpiContext::new(
        //     self.token_program.to_account_info(),
        //     initialize_transfer_hook_ix,
        // );

        // transfer_hook_initialize(
        //     transfer_hook_context,
        //     Some(self.user.key()),
        //     Some(crate::ID),
        // )?;

        // msg!("initialized the transfer hook");

        // let initialize_mint_accounts = InitializeMint2 {
        //     mint: self.mint.to_account_info(),
        // };

        // let initialize_mint_context = CpiContext::new(
        //     self.token_program.to_account_info(),
        //     initialize_mint_accounts,
        // );

        // initialize_mint2(initialize_mint_context, 9, &self.user.key(), None)?;

        // all instructions are handled by anchor account helpers

        let mint_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            MintToChecked {
                authority: self.user.to_account_info(),
                mint: self.mint.to_account_info(),
                to: self.source_token_account.to_account_info(),
            },
        );

        mint_to_checked(mint_ctx, 100_000_000_000, 9)?;

        Ok(())
    }
}
