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