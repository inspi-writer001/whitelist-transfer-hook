use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    pub address: Pubkey,
    pub is_whitelisted: bool, // this causes bottlenecks i.e - i keep adding PubKeys to this Vec - 300,000. addreses[299,999]
    pub bump: u8,
}
