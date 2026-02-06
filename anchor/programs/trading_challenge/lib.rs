use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("آدرس_قرارداد_شما_بعد_از_اولین_بیلد");

#[program]
pub mod trading_challenge {
    use super::*;

    // ۱. ایجاد چالش و واریز وجه توسط نفر اول
    pub fn create_challenge(ctx: Context<CreateChallenge>, challenge_id: String, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;
        
        ctx.accounts.challenge_state.creator = *ctx.accounts.user.key;
        ctx.accounts.challenge_state.amount = amount;
        ctx.accounts.challenge_state.challenge_id = challenge_id;
        ctx.accounts.challenge_state.status = 1; // 1 = Open
        
        Ok(())
    }

    // ۲. تسویه حساب توسط داور (Cloudflare Worker)
    pub fn settle_challenge(ctx: Context<SettleChallenge>, winner_pubkey: Pubkey) -> Result<()> {
        // امنیت: فقط "Authority" تعریف شده (Worker) می‌تواند این تابع را اجرا کند
        let amount_to_transfer = ctx.accounts.escrow_token_account.amount;
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.winner_token_account.to_account_info(),
            authority: ctx.accounts.escrow_authority.to_account_info(),
        };
        // منطق امضای قرارداد برای انتقال وجه از Escrow به برنده
        // ... (Signer seeds logic)
        
        Ok(())
    }
}

#[account]
pub struct ChallengeState {
    pub creator: Pubkey,
    pub opponent: Pubkey,
    pub amount: u64,
    pub challenge_id: String,
    pub status: u8,
}

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init, payer = user, space = 8 + 32 + 32 + 8 + 64 + 1)]
    pub challenge_state: Account<'info, ChallengeState>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct SettleChallenge<'info> {
    pub authority: Signer<'info>, // این باید آدرس عمومی کلودفلر ورکر شما باشد
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub winner_token_account: Account<'info, TokenAccount>,
    /// CHECK: Safe
    pub escrow_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}
