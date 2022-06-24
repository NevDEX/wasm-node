use web3::types::{H160,U256};
use tokio::runtime::Runtime;
use web3;
use marine_rs_sdk::marine;

extern crate hex;

struct Balance {
    url : String,
    account : String,
}

impl Balance {
    fn new(url : &str , account : &str) -> Balance {
        Balance { url: String::from(url), account: String::from(account) }
    }
    async fn balance(&self) -> web3::Result<U256> {
        let hex_account = hex::decode(&self.account).unwrap();
        let address = H160(to_array(hex_account.as_slice()));
        let transport = web3::transports::Http::new(&self.url)?;
        let web3 = web3::Web3::new(transport);
        let balance = web3.eth().balance(address, None).await?;
        Ok(balance)
    }
}

#[marine]
pub fn get_balance(url : &str , account : &str) -> u64 {
    let rtrun_time = Runtime::new().unwrap();
    let balance = rtrun_time.block_on(Balance::new(url,account).balance()).unwrap();
    balance.low_u64()
}

fn to_array(bytes: &[u8]) -> [u8; 20] {
    let mut array = [0; 20];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}