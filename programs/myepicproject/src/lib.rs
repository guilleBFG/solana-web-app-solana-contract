use anchor_lang::prelude::*;

declare_id!("ASMnwXRGMc76AVPsEraz9cMdMGMcRrRTEDR8FdGp4yEZ");


#[program]

pub mod myepicproject{
    use  super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) ->Result <()>{
        // get reference of the account
        let base_account = &mut ctx.accounts.base_account;
        // initialize variable
        base_account.total_gifs = 0;
        Ok(())
    }

    // add gifs
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()>{
        // get reference to the account and increase total figs
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }    
}

//attach certain variables to the start stuff context 
#[derive(Accounts)]
pub struct StartStuffOff <'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info,System>,
}

//specify what data you want in the add gifs context 
// getting a handle on the flow of things 

#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}


//create a custom structure for us to work with 
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct{
    pub gif_link: String,
    pub user_address: Pubkey,
}


//tell solana what we want to store on this account
#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}

