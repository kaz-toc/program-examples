use anchor_lang::prelude::*;
declare_id!("HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1");
#[program]
pub mod realloc_program {
    use super::*;
    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        ctx.accounts.state.value = 0;
        Ok(())
    }
    pub fn update(ctx: Context<UpdateContext>, value: i64) -> Result<()> {
        ctx.accounts.state.value = value;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(init, payer = user, space = 16, seeds = [b"realloc"], bump)]
    pub state: Account<'info, MessageState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct UpdateContext<'info> {
    #[account(mut, seeds = [b"reallod"], bump)]
    pub state: Account<'info, MessageState>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct MessageState {
    pub value: i64,
}
