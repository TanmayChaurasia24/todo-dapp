use anchor_lang::prelude::*;

use crate::states::UserProfile;
mod states;
mod constants;
mod error;

declare_id!("DUsxLHSAaPKGNxoLPHnjAx16BZ88LXs9AwVn9fkN56Cj");

#[program]
pub mod todocontract {
    use anchor_lang::*;

    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        msg!("Initialized the account for: {:?}", ctx.program_id);

        let user_profile: &mut Box<Account<'_, UserProfile>> = &mut ctx.accounts.user_profile;  
        msg!("User profile: {:#?}", user_profile);

        user_profile.authority = ctx.accounts.authority.key();
        user_profile.todo_count = 0;
        user_profile.last_todo = 0;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>, // wallet or user who is starting the transaction, Signer ensures that user has signed the trans

    // creating the pda
    #[account(
        init, // tells that account will be created during the transaction
        seeds = [constants::USER_TAG, authority.key().as_ref()], // used to create the unique address on solana, seeds get combined with program id and create the derived address which is used to represent accounts control by the specific program.
        bump, // some combinations of seeds and program id result in public keys that lies on elliptic curve by solana, bumps verify that pda is valid
        payer = authority, // this means the user who is signing the transaction will pay the gas fee
        space = 8 + std::mem::size_of::<UserProfile>() // reserve this much space on the account.
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System> // it is req to create accounts on solana
}
