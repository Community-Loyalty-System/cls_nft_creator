use anchor_lang::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::fs::File;
use std::io::Read;

#[account]
#[derive(Default)]
pub struct NftAccount {
    pub mint_authority: Pubkey,
    pub data: Vec<u8>,
}

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(
        init,
        seeds = ["my-nft".as_bytes()],
        payer = user,
        space = 8 + 32 + 32 + 32 + 32 + 1 + 2000,
    )]
    pub nft_account: ProgramAccount<'info, NftAccount>,
    #[account(mut)]
    pub nft_mint: AccountInfo<'info>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

pub fn create_nft(
    ctx: Context<CreateNft>,
    image: String,
) -> ProgramResult {
    let data = read_image(&image)?;

    let mut nft_account = &mut ctx.accounts.nft_account;
    nft_account.mint_authority = *ctx.accounts.user.key;
    nft_account.data = data;

    let seeds = &["my-nft".as_bytes(), &[nft_account.data[0]]];
    let (nft_mint_authority, _) =
        Pubkey::find_program_address(seeds, &ctx.program_id);

    let cpi_accounts = mint_to(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.nft_mint.to_account_info(),
        ctx.accounts.user.to_account_info(),
        1,&[])?;
    
    }