use anchor_lang::prelude::*;

declare_id!("EqaKtMpHuhqRbakvfMGoLXHZsV6dCD3DRp6uqEU5pbVq");

#[program]
pub mod todo_app_solana_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
