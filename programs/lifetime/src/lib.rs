use anchor_lang::prelude::*;

declare_id!("52XbisxVLPiSTV7voRXMkRjX31yw9XwZJJPQSi81tVkV");

#[program]
pub mod lifetime {
    use super::*;

    pub fn do_thing<'info>(ctx: Context<'_, 'info, 'info, '_, DoThing<'info>>) -> Result<()> {
        handle(ctx)
    }
}

#[account]
#[derive(Default)]
pub struct Account1 {
    pub value: String,
}

// #[derive(Accounts)]
// pub struct DoThing<'info> {
//     pub account1: Account<'info, Account1>,
// }

// pub fn handle(ctx: Context<DoThing>) -> Result<()> {
//     let account1 = &ctx.accounts.account1;

//     msg!("other_account.value: {}", account1.value);

//     Ok(())
// }

#[derive(Accounts)]
pub struct DoThing<'info> {
    pub account1: UncheckedAccount<'info>,
}

pub fn handle<'info>(ctx: Context<'_, 'info, 'info, '_, DoThing<'info>>) -> Result<()> {
    let account1 = Account::<Account1>::try_from(&ctx.accounts.account1)?;

    msg!("other_account.value: {}", account1.value);

    Ok(())
}
