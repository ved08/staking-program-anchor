use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("CpYAJVZiC971BWuxGzwSjBnHC4TZFbkKtBy86F9gE5p6");

const ANCHOR_MINT_ADDRESS: &str = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";

#[program]
pub mod stake_anchor {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.pool.authority = ctx.accounts.authority.key();
        ctx.accounts.pool.user_count = 0u32;
        Ok(())
    }
    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.stake = 0u64;
        user.bump = *ctx.bumps.get("user").unwrap();
        ctx.accounts.pool.user_count += 1;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            token::Transfer {
                from: ctx.accounts.user_anchor_ata.to_account_info(),
                authority: ctx.accounts.user_anchor_ata_authority.to_account_info(),
                to: ctx.accounts.program_anchor_ata.to_account_info(),
            }); 
            token::transfer(cpi_ctx, amount)?;
            ctx.accounts.user.stake += amount;
            ctx.accounts.pool.total_staked += amount;
        Ok(())
    }
}

pub const POOL_STORAGE_TOTAL_BYTES: usize = 32 + 4;
#[account]
pub struct Pool {
    pub authority: Pubkey,
    pub user_count: u32,
    pub total_staked: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + POOL_STORAGE_TOTAL_BYTES)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

pub const USER_STORAGE_TOTAL_BYTES: usize = 1+8;
#[account]
pub struct User {
    bump: u8,
    stake: u64
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer=authority,space=8+USER_STORAGE_TOTAL_BYTES, seeds=[b"user", authority.key().as_ref()],
    bump
    )]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Stake<'info> {
    // span class="grey"
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(
        mut,
        seeds=[b"user", user_anchor_ata_authority.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    #[account(
        address = ANCHOR_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    pub anchor_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_anchor_ata: Account<'info, TokenAccount>,
    pub user_anchor_ata_authority: Signer<'info>,
    #[account(mut)]
    pub program_anchor_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>
}



