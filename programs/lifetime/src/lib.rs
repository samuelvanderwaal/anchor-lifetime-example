use anchor_lang::prelude::*;

declare_id!("52XbisxVLPiSTV7voRXMkRjX31yw9XwZJJPQSi81tVkV");

#[program]
pub mod lifetime {
    use super::*;

    pub fn do_thing<'c: 'info, 'info>(
        ctx: Context<'_, 'c, 'info, 'info, DoThing<'info>>,
    ) -> Result<()> {
        let account1 = Account::<Account1>::try_from(&ctx.accounts.account1)?;

        msg!("other_account.value: {}", account1.value);

        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct Account1 {
    pub value: String,
}

#[derive(Accounts)]
pub struct DoThing<'info> {
    pub account1: UncheckedAccount<'info>,
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
