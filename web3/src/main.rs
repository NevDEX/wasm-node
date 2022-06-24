use marine_rs_sdk::marine;

mod balance;
use balance::{get_account,Accounts};
mod account;
mod transction;

fn main() {
    transction::get_contract_owner("http://127.0.0.1:7545", "0e7760FBD54b13D6453b2fAC815070c7DCCeafbB");
}

#[marine]
pub fn get_accounts(url : String)  -> Accounts{
    return get_account(url)
}

#[marine]
pub fn get_balance(url : String , account: String) -> u64 {
    return  account::get_balance(&url, &account)
}