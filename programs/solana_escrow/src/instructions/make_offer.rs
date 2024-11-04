use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};


use crate::{Offer, ANCHOR_DISCRIMINATOR};


#[derive(Accounts)]
#[instruction(id: u64)] // What's this instruction doing here? | This specifies the instruction per program?
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, // This `Signer` type defines, who's going to pay fees?

    #[account(mint::token_program = token_program)]
    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub token_mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = maker,
        space = ANCHOR_DISCRIMINATOR + Offer::INIT_SPACE,
        seeds = [b"offer", maker.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub offer: Account<'info, Offer>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    // I believe, System Program is required here since we create an account per offer.
    // Accounts are created via `SystemProgram`
    pub system_program: Program<'info, System>,
}