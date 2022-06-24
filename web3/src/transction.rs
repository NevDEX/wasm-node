// 0x0e7760FBD54b13D6453b2fAC815070c7DCCeafbB
use web3::{
    types::{H160},
    contract::{Contract, Options},
};
// use tokio::runtime::Runtime;
use web3;
use tokio::runtime::Runtime;


struct TTs {
    url : String,
    contract : String,
}

impl TTs {
    fn new(url:&str , contract : &str ) -> TTs {
        TTs {
            url : url.to_string(),
            contract : contract.to_string(),
        }
    }
    async fn get_owner(&self) -> web3::contract::Result<()> {
        let transport = web3::transports::Http::new(&self.url)?;
        let web3 = web3::Web3::new(transport);

        let hex_contract = hex::decode(&self.contract).unwrap();
        let contract_address = H160(to_array(hex_contract.as_slice()));

        let hex_address = hex::decode("fB5F97301c5c695E576E65FAD86eCd360d768243").unwrap();
        let account_address = H160(to_array(hex_address.as_slice()));

        let contract = Contract::from_json(
            web3.eth(),
            contract_address,
            include_bytes!("./abis/testing.abi"),
        ).unwrap();
        let abi = contract.call("getOwner",(),account_address,Options::default()).await?;
        println!("{}" , abi);
        Ok(())
    }
}

pub fn get_contract_owner(url:&str , contract : &str ) {
    let rt = Runtime::new().unwrap();
   rt.block_on(TTs::new(url, contract).get_owner()).unwrap();
}

fn to_array(bytes: &[u8]) -> [u8; 20] {
    let mut array = [0; 20];
    let bytes = &bytes[..array.len()];
    array.copy_from_slice(bytes);
    array
}