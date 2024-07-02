use anchor_lang::prelude::*;

declare_id!("BGpu8rxPV3RgjrDwtnJPCbn2ohvQ9LcAXWhTzWS9LDP7");

#[program]
pub mod hello_world {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
       
       let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;

       base_account.greeting = "Hello, World".to_string();

       Ok(())
    }

    pub fn update_greeting(ctx: Context<UpdateGreeting>, greeting: String) -> ProgramResult {

        let base_account: &mut Account<BaseAccount> = &mut ctx.accounts.base_account;

        base_account.greeting = greeting;

        Ok(())  
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8+32)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGreeting<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[account]
pub struct BaseAccount {
    pub greeting: String,
}




