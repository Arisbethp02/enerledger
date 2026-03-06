use anchor_lang::prelude::*;

declare_id!("HiTjBaHUp7t6ETtZNEZdfKJ7gCTSAJn5uY8jzgNHsE6j");

#[program]
pub mod enerledger {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
