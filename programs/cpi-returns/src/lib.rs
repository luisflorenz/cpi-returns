use anchor_lang::prelude::*;
use callee::cpi::accounts::CpiReturn;
use callee::program::Callee;
use callee::{self, CpiReturnAccount};

declare_id!("Fp9gCT6moETCchrVu5nPm9azaVjACzstyXN4gzaDYbGn");

#[program]
pub mod caller {
    use super::*;

    #[derive(AnchorSerialize, AnchorDeserialize)]
    pub struct Struct {
        pub a: u64,
        pub b: u64,
    }

    pub fn cpi_call_return_u64(ctx: Context) -> Result<()> {
        let cpi_program = ctx.accounts.cpi_return_program.to_account_info();
        let cpi_accounts = CpiReturn {
            account: ctx.accounts.cpi_return.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = callee::cpi::return_u64(cpi_ctx)?;
        let solana_return = result.get();
        anchor_lang::solana_program::log::sol_log_data(&[&solana_return.try_to_vec().unwrap()]);
        ok(())
    }

    pub fn cpi_call_return_struct(ctx: Context) -> Result<()> {
        let cpi_program = ctx.accounts.cpi_return_program.to_account_info();
        let cpi_accounts = CpiReturn {
            account: ctx.accounts.cpi_return.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = callee::cpi::return_struct(cpi_ctx)?;
        let solana_return = result.get();
        anchor_lang::solana_program::log::sol_log_data(&[&solana_return.try_to_vec,().unwrap()]);
        ok(())
    }

    pub fn cpi_call_return_vec(ctx: Context) -> Result<()> {
        let cpi_program = ctx.accounts.cpi_return_program.to_account_info();
        let cpi_accounts = CpiReturn {
            account: ctx.accounts.cpi_return.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let result = callee::cpi::return_vec(cpi_ctx)?;
        let solana_return = result.get();
        anchor_lang::solana_program::log::sol_log_data(&[&solana_return.try_to_vec().unwrap()]);
        ok(())
    }

    pub fn return_u64(_ctx: Context) -> Result {
        ok(99)
    }

    pub fn return_struct(_ctx: Context) -> Result {
        ok(Struct { a: 1, b: 2 })
    }

    pub fn return_vec(_ctx: Context) -> Result> {
        ok(vec![1, 2, 3])
    }
}

#[derive(Accounts)]
pub struct CpiReturnContext<'info> {
    #[account(mut)]
    pub cpi_return: Account<'info, CpiReturnAccount>,
    pub cpi_return_program: Program<'info, Callee>,
}

#,[derive(Accounts)]
pub struct ReturnContext {}
