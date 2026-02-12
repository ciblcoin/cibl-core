use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("C1BL111111111111111111111111111111111111111");

#[program]
pub mod cibl_gamble {
    use super::*;

    pub fn create_challenge(ctx: Context<CreateChallenge>, amount: u64) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        challenge.creator = *ctx.accounts.creator.key;
        challenge.amount = amount;
        challenge.status = String::from("pending");

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

        msg!("Challenge created successfully");
        Ok(())
    }

    pub fn resolve_challenge(ctx: Context<ResolveChallenge>, winner_share: u64, fee_share: u64) -> Result<()> {
        let challenge = &mut ctx.accounts.challenge;
        
        **ctx.accounts.escrow_account.try_borrow_mut_lamports()? -= winner_share + fee_share;
        **ctx.accounts.winner.try_borrow_mut_lamports()? += winner_share;
        **ctx.accounts.fee_receiver.try_borrow_mut_lamports()? += fee_share;

        challenge.status = String::from("completed");
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
    #[account(init, payer = creator, space = 8 + 32 + 8 + 64)]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub creator: Signer<'info>,
    /// CHECK: Escrow Vault
    #[account(mut)]
    pub escrow_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveChallenge<'info> {
    #[account(mut)]
    pub challenge: Account<'info, Challenge>,
    /// CHECK: Winner Wallet
    #[account(mut)]
    pub winner: AccountInfo<'info>,
    /// CHECK: CiBL Fee Wallet
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,
    #[account(mut)]
    pub escrow_account: AccountInfo<'info>,
}
