// src/cpi.rs
use crate::instructions::InitializeBumps;
use anchor_lang::prelude::*;

pub mod accounts {
    pub use crate::*;
}

pub mod instructions {

    use super::*;

    pub fn initialize<'info>(
        mut ctx: CpiContext<'_, '_, '_, 'info, crate::Initialize<'info>>,
        bumps: InitializeBumps,
        init_amount_0: u64,
        init_amount_1: u64,
        open_time: u64,
    ) -> Result<()> {
        crate::instructions::initialize(
            Context::new(
                ctx.program.key,
                &mut ctx.accounts,
                &ctx.remaining_accounts,
                bumps,
            ),
            init_amount_0,
            init_amount_1,
            open_time,
        )
    }
}
