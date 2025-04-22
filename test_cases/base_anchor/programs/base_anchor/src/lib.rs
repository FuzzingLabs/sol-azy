use anchor_lang::prelude::*;
use anchor_lang::solana_program::program_pack::Pack;
use spl_token::state::Account as SplTokenAccount;

declare_id!("5fdvcJ2tsUv4Ei2G49RPTA5ArV8cQ7babW85tDJX9Y2u");

#[program]
pub mod base_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn log_message(ctx: Context<LogMessage>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        msg!("Your account balance is: {}", token.amount);
        SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        Ok(())
    }

    pub fn log_message2(ctx: Context<LogMessage>) -> Result<()> {
        let token = SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        msg!("Your account balance is: {}", token.amount);
        SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        SplTokenAccount::unpack(&ctx.accounts.token.data.borrow())?;
        Ok(())
    }

    // ❌ Bad
    // Reallocating without zero-initialization
    pub fn update_bad_1(ctx: Context<UpdateBad1>) -> Result<()> {
        let new_size = 0x2600;
        let current_data_size = ctx.accounts.authority.data.borrow().len();
        if current_data_size < new_size {
            ctx.accounts.authority.realloc(new_size, false)?;
        }
        Ok(())
    }

    // ❌ Bad
    // Reallocating twice to test SAST rules
    pub fn update_bad_2(ctx: Context<UpdateBad2>) -> Result<()> {
        let new_size = 0x2600;
        let current_data_size = ctx.accounts.authority.data.borrow().len();
        if current_data_size < new_size {
            ctx.accounts.authority.realloc(new_size, true)?;
        }
        if current_data_size < new_size {
            ctx.accounts.authority.realloc(new_size, false)?;
        }
        Ok(())
    }

    // ✅ Good
    // Safely reallocating with proper memory management
    pub fn update_good_1(ctx: Context<UpdateGood1>) -> Result<()> {
        let new_size = 0x2600;
        let current_data_size = ctx.accounts.authority.data.borrow().len();
        ctx.accounts.authority.realloc(new_size, false)?;
        if current_data_size < new_size {
            let data = &mut ctx.accounts.authority.data.borrow_mut();
            for i in current_data_size..new_size {
                data[i] = 0;
            }
        }
        Ok(())
    }

    // ✅ Good
    // Safely reallocating with proper memory management
    pub fn update_good_2(ctx: Context<UpdateGood2>) -> Result<()> {
        ctx.accounts.authority.realloc(0x15, true)?;
        Ok(())
    }

    // ✅ Good
    // Safely reallocating with proper memory management
    pub fn update_good_3(ctx: Context<UpdateGood3>) -> Result<()> {
        let new_size = 0x2600;
        let current_data_size = ctx.accounts.authority.data.borrow().len();
        if current_data_size < new_size {
            ctx.accounts.authority.realloc(new_size, true)?;
        }
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct LogMessage<'info> {
    token: AccountInfo<'info>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateBad1<'init> {
    authority: AccountInfo<'init>,
}

#[derive(Accounts)]
pub struct UpdateBad2<'init> {
    authority: AccountInfo<'init>,
}


#[derive(Accounts)]
pub struct UpdateGood1<'init> {
    authority: AccountInfo<'init>,
}

#[derive(Accounts)]
pub struct UpdateGood2<'init> {
    authority: AccountInfo<'init>,
}

#[derive(Accounts)]
pub struct UpdateGood3<'init> {
    authority: AccountInfo<'init>,
}