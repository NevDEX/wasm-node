use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use tokio::runtime::Runtime;
use web3;
use web3::types::Address;


module_manifest!();

struct GetAccount {
    url: String,
}

#[marine]
pub struct Accounts {
    pub list: Vec<String>,
}

impl GetAccount {
    fn new(url: &str) -> GetAccount {
        GetAccount {
            url: url.to_string(),
        }
    }
    async fn get_accounts(&self) -> web3::Result<Vec<Address>> {
        let transport = web3::transports::Http::new(&self.url)?;
        let web3 = web3::Web3::new(transport);
        let accounts = web3.eth().accounts().await?;
        Ok(accounts)
    }
}


pub fn get_account(url: String) -> Accounts {
    let rt = Runtime::new().unwrap();
    let requst = GetAccount::new(&url);
    let accounts = rt.block_on(requst.get_accounts()).unwrap();
    let mut ret_list: Vec<String> = Vec::new();
    for value in accounts {
        ret_list.push(value.to_string());
    }
    Accounts { list: ret_list }
}