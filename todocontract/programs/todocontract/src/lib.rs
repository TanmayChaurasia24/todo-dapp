mod states;
mod constants;
mod error;

use anchor_lang::prelude::*;
use crate::states::{TodoAccount, UserProfile};
use crate::error::TodoError;

declare_id!("DUsxLHSAaPKGNxoLPHnjAx16BZ88LXs9AwVn9fkN56Cj");

#[program]
pub mod todocontract {
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

    pub fn add_todo(ctx: Context<AddTodo>, todo_idx: u8, content: String) -> Result<()> {
        msg!("Adding todo to: {:?}", ctx.accounts.todo_account);

        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = todo_idx;
        todo_account.content = content;
        todo_account.marked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn mark_todo(ctx: Context<MarkTodo>, _todo_idx: u8) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;

        require!(!todo_account.marked, TodoError::AlreadyMarked); // Fix: should be NOT marked

        todo_account.marked = true;
        Ok(())
    }

    pub fn remove_todo(ctx: Context<RemoveTodo>, _todo_idx: u8) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

        // Todo account is closed automatically with `close = authority`
        Ok(())
    }
}


#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [constants::USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
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
        seeds = [constants::TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<TodoAccount>(),
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct MarkTodo<'info> {
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
        mut,
        seeds = [constants::TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct RemoveTodo<'info> {
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
        mut,
        close = authority,
        seeds = [constants::TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>,
}
