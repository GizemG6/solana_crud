use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!(""); 

#[program]
mod solana_crud {
    use super::*;
    pub fn create(
        ctx: Context<Create>,
        image: String,
        score: u64,
        age: u64,
        name: String,
        uuid: String,
        graduated: bool,
    ) -> Result<()> {
        ctx.accounts.data_account.set_inner(SomeDataAccount {
            uuid: uuid,
            name: name.clone(),
            age: age,
            graduated: graduated,
            score: score,
            image: image,
            owner: ctx.accounts.signer.key(),
            bump: ctx.bumps.data_account,
        });

        msg!("Kayit gerceklesti {}!", name);

        Ok(())
    }
    pub fn update(
        ctx: Context<Update>,
        image: String,
        score: u64,
        age: u64,
        name: String,
        _uuid: String,
        graduated: bool,
    ) -> Result<()> {
        ctx.accounts.data_account.age = age;
        ctx.accounts.data_account.name = name.clone();
        ctx.accounts.data_account.graduated = graduated;
        ctx.accounts.data_account.score = score;
        ctx.accounts.data_account.image = image;

        msg!("Guncellendi");
        Ok(())
    }
    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        let data: &mut Account<SomeDataAccount> = &mut ctx.accounts.data_account;
        let owner: &Signer = &ctx.accounts.signer;
        data.close(owner.to_account_info());
        msg!("Hesap silindi");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(
        init, 
        payer = signer, 
        space = 8 + 8 + (4 + 50) + 8 + 1 + 8 + (4 + 100) + 32 + 1,
        bump,
        seeds = [b"data",signer.key.as_ref()]
    )]
    pub data_account: Account<'info, SomeDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(
        mut,
        seeds = [b"data",signer.key.as_ref()],
        bump = data_account.bump
    )]
    pub data_account: Account<'info, SomeDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(
        mut,
        seeds = [b"data",signer.key.as_ref()],
        bump = data_account.bump
    )]
    pub data_account: Account<'info, SomeDataAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SomeDataAccount {
    pub uuid: String,
    pub name: String,
    pub age: u64,
    pub graduated: bool,
    pub score: u64,
    pub image: String,
    pub owner: Pubkey,
    pub bump: u8,
}
