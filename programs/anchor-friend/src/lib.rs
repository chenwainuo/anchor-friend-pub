use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

declare_id!("CeQk2BBaT2rua9mmMKWkwyCY6JcdLQND2wdYGeVEp9TT");

#[program]
pub mod anchor_friend {
    use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
    use super::*;

    pub fn init_admin(ctx: Context<InitState>, bump: u8) -> Result<()> {
        ctx.accounts.state.admin = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn init_vault(ctx: Context<InitVault>, bump: u8) -> Result<()> {
        Ok(())
    }


    pub fn init_owner_share_state(ctx: Context<InitOwnerShareState>, bump: u8, state_bump: u8) -> Result<()> {
        ctx.accounts.owner_share_state.owner = ctx.accounts.owner_pubkey.key();
        ctx.accounts.owner_share_state.social_media_handle = ctx.accounts.social_media_handle.key();
        ctx.accounts.owner_share_state.bump = bump;
        Ok(())
    }

    pub fn init_holding(ctx: Context<InitOwnerHolding>, bump: u8, state_bump: u8) -> Result<()> {
        ctx.accounts.holding.shares = 1;
        ctx.accounts.owner_share_state.shares = 1;
        Ok(())
    }

    pub fn buy_holding(ctx: Context<TransactHoldings>, bump: u8, vault_bump: u8, state_bump: u8, old_share: u16, k: u64) -> Result<()> {
        msg!("current share {}", ctx.accounts.owner_share_state.shares);
        if old_share != ctx.accounts.owner_share_state.shares {
            msg!("front ran");
            panic!()
        }

        let supply = old_share as u64;
        let temp1 = supply.clone().checked_sub(1).unwrap();
        let temp2 = (2 as u64).checked_mul(temp1.clone()).unwrap().checked_add(1).unwrap();

        let sum1 = temp1.clone()
            .checked_mul(supply)
            .unwrap()
            .checked_mul(temp2)
            .unwrap()
            .checked_div(6)
            .unwrap();

        let temp3 = temp1.checked_add(k.clone()).unwrap();
        let temp4 = supply.clone().checked_add(k.clone()).unwrap();
        let temp5 = (2 as u64).checked_mul(temp3.clone()).unwrap().checked_add(1).unwrap();

        let sum2 = temp3
            .checked_mul(temp4)
            .unwrap()
            .checked_mul(temp5)
            .unwrap()
            .checked_div(6)
            .unwrap();
        let summation: u64 = (sum2.checked_sub(sum1)).unwrap() as u64;
        let price = (summation * LAMPORTS_PER_SOL).checked_div(16000).unwrap();
        let owner_fee = price.checked_mul(50000000).unwrap().checked_div(LAMPORTS_PER_SOL).unwrap();
        let protocol_fee = price.checked_mul(50000000).unwrap().checked_div(LAMPORTS_PER_SOL).unwrap();

        {
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &ctx.accounts.vault.key(),
                price,
            );

            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.vault.to_account_info(),
                ],
            );
            msg!("transferred {} as price for key", price);
        }

        {
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &ctx.accounts.owner_pubkey.key(),
                owner_fee,
            );

            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.owner_pubkey.to_account_info(),
                ],
            );
            msg!("transferred {} as owner fee for key", owner_fee);
        }

        {
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &ctx.accounts.admin.key(),
                protocol_fee,
            );

            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            );
            msg!("transferred {} as protocol fee for key", protocol_fee);
        }


        msg!("price {} supply {} k {}", price, old_share, k);
        ctx.accounts.owner_share_state.shares = ctx.accounts.owner_share_state.shares.checked_add(k.clone() as u16).unwrap();
        ;
        ctx.accounts.holding.shares = ctx.accounts.holding.shares.checked_add(k.clone() as u16).unwrap();
        Ok(())
    }

    pub fn sell_holding(ctx: Context<TransactHoldings>, bump: u8, vault_bump: u8, state_bump: u8, old_share: u16, k: u64) -> Result<()> {
        msg!("current share {}", ctx.accounts.owner_share_state.shares);
        if old_share != ctx.accounts.owner_share_state.shares {
            msg!("front ran");
            panic!()
        }
        if ctx.accounts.owner_share_state.shares == 0 || ctx.accounts.holding.shares == 0 {
            msg!("out of shares to sell, total {}, you own {} ", ctx.accounts.owner_share_state.shares, ctx.accounts.holding.shares);
            panic!()
        }
        let supply = old_share as u64;
        let temp1 = supply.clone().checked_sub(1).unwrap();
        let temp2 = (2 as u64).checked_mul(temp1.clone()).unwrap().checked_add(1).unwrap();

        let sum1 = temp1.clone()
            .checked_mul(supply)
            .unwrap()
            .checked_mul(temp2)
            .unwrap()
            .checked_div(6)
            .unwrap();

        let temp3 = temp1.checked_add(k.clone()).unwrap();
        let temp4 = supply.clone().checked_add(k.clone()).unwrap();
        let temp5 = (2 as u64).checked_mul(temp3.clone()).unwrap().checked_add(1).unwrap();

        let sum2 = temp3
            .checked_mul(temp4)
            .unwrap()
            .checked_mul(temp5)
            .unwrap()
            .checked_div(6)
            .unwrap();
        let summation: u64 = (sum2.checked_sub(sum1)).unwrap() as u64;
        let price = (summation * LAMPORTS_PER_SOL).checked_div(16000).unwrap();

        let owner_fee = price.checked_mul(50000000).unwrap().checked_div(LAMPORTS_PER_SOL).unwrap();
        let protocol_fee = price.checked_mul(50000000).unwrap().checked_div(LAMPORTS_PER_SOL).unwrap();
        let cpi_program = ctx.accounts.system_program.to_account_info();

        {
            let cpi_accounts = Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            };

            let signature_seeds = [
                b"vault".as_ref(),
                &[vault_bump],
            ];
            let signers = &[&signature_seeds[..]];

            let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, signers);

            transfer(cpi_context, price)?;
            msg!("pay out {} as price for key", price);
        }


        {
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &ctx.accounts.owner_pubkey.key(),
                owner_fee,
            );

            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.owner_pubkey.to_account_info(),
                ],
            );
            msg!("transferred {} as owner fee for key", owner_fee);
        }

        {
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &ctx.accounts.signer.key(),
                &ctx.accounts.admin.key(),
                protocol_fee,
            );

            anchor_lang::solana_program::program::invoke(
                &ix,
                &[
                    ctx.accounts.signer.to_account_info(),
                    ctx.accounts.admin.to_account_info(),
                ],
            );
            msg!("transferred {} as protocol_fee fee for key", protocol_fee);
        }

        msg!("price {} supply {} k {}", price, old_share, k);
        ctx.accounts.owner_share_state.shares = ctx.accounts.owner_share_state.shares.checked_sub(k.clone() as u16).unwrap();
        ctx.accounts.holding.shares = ctx.accounts.holding.shares.checked_sub(k.clone() as u16).unwrap();
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct State {
    pub admin: Pubkey,
}


#[account]
#[derive(Default)]
pub struct OwnerShareState {
    pub owner: Pubkey,
    pub social_media_handle: Pubkey,
    pub shares: u16,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct Holding {
    pub shares: u16,
}


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitState<'info> {
    #[account(init, seeds = [b"state"], bump, payer = signer, space = std::mem::size_of::< State > () + 8)]
    pub state: Account<'info, State>,
    /// Solana Stuff
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitVault<'info> {
    /// Solana Stuff
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts, Clone)]
#[instruction(bump: u8, state_bump: u8)]
pub struct InitOwnerShareState<'info> {
    #[account(init, seeds = [b"owner_share_state", owner_pubkey.key.as_ref()], bump, payer = signer, space = std::mem::size_of::< OwnerShareState > () + 8)]
    pub owner_share_state: Account<'info, OwnerShareState>,
    #[account(seeds = [b"state"], bump = state_bump)]
    pub state: Account<'info, State>,
    /// CHECK owner pubkey
    pub owner_pubkey: AccountInfo<'info>,
    /// CHECK social media pda
    pub social_media_handle: AccountInfo<'info>,
    // Solana stuff
    #[account(mut, constraint = signer.key().as_ref() == state.admin.key().as_ref())]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts, Clone)]
#[instruction(bump: u8, state_bump: u8)]
pub struct InitOwnerHolding<'info> {
    #[account(mut, seeds = [b"owner_share_state", owner_pubkey.key.as_ref()], bump)]
    pub owner_share_state: Account<'info, OwnerShareState>,
    #[account(seeds = [b"state"], bump = state_bump)]
    pub state: Account<'info, State>,
    #[account(init, seeds = [b"holding", owner_pubkey.key.as_ref(), owner_pubkey.key.as_ref()], bump, payer = signer, space = std::mem::size_of::< Holding > () + 8)]
    pub holding: Account<'info, Holding>,
    /// CHECK owner pubkey
    pub owner_pubkey: AccountInfo<'info>,
    #[account(mut, constraint = signer.key().as_ref() == state.admin.key().as_ref())]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts, Clone)]
#[instruction(bump: u8, vault_bump: u8, state_bump: u8)]
pub struct TransactHoldings<'info> {
    #[account(mut, seeds = [b"owner_share_state", owner_pubkey.key.as_ref()], bump = owner_share_state.bump)]
    pub owner_share_state: Account<'info, OwnerShareState>,
    #[account(init_if_needed, seeds = [b"holding", owner_pubkey.key.as_ref(), signer.key.as_ref()], bump, payer = signer, space = std::mem::size_of::< Holding > () + 8)]
    pub holding: Account<'info, Holding>,
    /// CHECK vault
    #[account(mut, seeds = [b"vault"], bump = vault_bump)]
    pub vault: AccountInfo<'info>,
    /// CHECK owner pubkey
    #[account(mut)]
    pub owner_pubkey: AccountInfo<'info>,
    #[account(seeds = [b"state"], bump = state_bump)]
    pub state: Account<'info, State>,
    /// CHECK admin key checked with state
    #[account(mut, constraint = admin.key().as_ref() == state.admin.key().as_ref())]
    pub admin: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
