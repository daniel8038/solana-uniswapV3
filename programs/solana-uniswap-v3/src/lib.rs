use anchor_lang::prelude::*;

declare_id!("8id74tiZuDwcqhDouagu2TTrpdjctNuFvdMNpcoLctog");

#[program]
pub mod solana_uniswap_v3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
