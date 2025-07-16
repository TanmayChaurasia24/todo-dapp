use anchor_lang::prelude::*;

use crate::states::{TodoAccount, UserProfile};
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

        // initialize the user profile
        let user_profile: &mut Box<Account<'_, UserProfile>> = &mut ctx.accounts.user_profile;  
        msg!("User profile: {:#?}", user_profile);

        // initialize the user profile with default data
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.todo_count = 0;
        user_profile.last_todo = 0;

        Ok(())
    }

    pub fn add_todo(ctx: Context<AddTodo>, _content: String) -> Result<()> {
        msg!("adding todo to: {:?}", ctx.accounts.todo_account);

        let todo_account: &mut Box<Account<'_, TodoAccount>> = &mut ctx.accounts.todo_account;
        let user_profile: &mut Box<Account<'_, UserProfile>> = &mut ctx.accounts.user_profile;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.content = _content;
        todo_account.marked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

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


#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [constants::USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [constants::TODO_TAG, authority.key().as_ref(), &[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>() + 8,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>

}