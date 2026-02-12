use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("آدرس_کانتراکت_شما_بعد_از_دیپلوی");

#[program]
pub mod cibl_gamble {
    use super::*;

    // 1. ایجاد یک چالش جدید
    pub fn create_challenge(ctx: Context<CreateChallenge>, amount: u64) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        challenge.creator = *ctx.accounts.creator.key;
        challenge.amount = amount;
        challenge.status = String::from("pending");

        // انتقال وجه از کیف پول کاربر به صندوق کانتراکت (Escrow)
        let ix = system_instruction::transfer(
            ctx.accounts.creator.key,
            ctx.accounts.escrow_account.key,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.creator.to_account_info(),
                ctx.accounts.escrow_account.to_account_info(),
            ],
        )?;

        msg!("Challenge created for {} lamports", amount);
        Ok(())
    }

    // 2. پایان چالش و توزیع جوایز (برنده، بازنده و کارمزد CiBL)
    pub fn resolve_challenge(ctx: Context<ResolveChallenge>, winner_share: u64, fee_share: u64) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        
        // انتقال سهم برنده
        **ctx.accounts.escrow_account.try_borrow_mut_lamports()? -= winner_share;
        **ctx.accounts.winner.try_borrow_mut_lamports()? += winner_share;

        // انتقال کارمزد CiBL (همان 0.6% یا سهم توافقی)
        **ctx.accounts.escrow_account.try_borrow_mut_lamports()? -= fee_share;
        **ctx.accounts.fee_receiver.try_borrow_mut_lamports()? += fee_share;

        challenge.status = String::from("completed");
        msg!("Challenge resolved. Winner paid, fee collected.");
        Ok(())
    }
}

#[account]
pub struct Challenge {
    pub creator: Pubkey,
    pub amount: u64,
    pub status: String,
}

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(init, picker = creator, space = 8 + 32 + 8 + 32)]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: این حساب صندوق امن است
    #[account(mut)]
    pub escrow_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveChallenge<'info> {
    #[account(mut)]
    pub challenge: Account<'info, Challenge>,
    /// CHECK: حساب برنده
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    /// CHECK: کیف پول شما برای دریافت کارمزد
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_account: AccountInfo<'info>,
}
