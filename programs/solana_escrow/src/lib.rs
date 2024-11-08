pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7zWLG6ewGojjqT1QuXXWaUU5f1JH3ve1EUHouDF358DR");

#[program]
pub mod solana_escrow {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        Ok(())
        // initialize::handler(ctx)
    }
}
