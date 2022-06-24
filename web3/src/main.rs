use marine_rs_sdk::marine;

mod balance;
use balance::{get_account,Accounts};
mod account;


fn main() {
  
}

#[marine]
pub fn get_accounts(url : String)  -> Accounts{
    return get_account(url)
}

#[marine]
pub fn get_balance(url : String , account: String) -> u64 {
    return  account::get_balance(&url, &account)
}