use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use std::time::SystemTime;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::collections::HashMap;
use orderbook::{Orderbook, OrderSide, orders};

module_manifest!();

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum BrokerAsset {
    USD,
    EUR,
    BTC,
    ETH,
}

// static INSTANCE: OnceCell<Mutex<HashMap<String,orderbook::Orderbook<BrokerAsset>>>> = OnceCell::new();
static ORDERBOOK: OnceCell<Mutex< orderbook::Orderbook<BrokerAsset>>> = OnceCell::new();

pub fn main() {


}

fn parse_asset(asset: &str) -> Option<BrokerAsset> {
    match asset {
        "USD" => Some(BrokerAsset::USD),
        "EUR" => Some(BrokerAsset::EUR),
        "BTC" => Some(BrokerAsset::BTC),
        "ETH" => Some(BrokerAsset::ETH),
        _ => None,
    }
}

pub fn get_orderbooks() -> &'static Mutex<orderbook::Orderbook<BrokerAsset>> {
    ORDERBOOK.get_or_init(|| {
        let mut ordrbook =   Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
        Mutex::new(ordrbook)
    })
}

#[marine]
pub fn greeting(name: String) -> String {
    format!("Hi, {}", name)
}

#[marine]
pub fn create_market() -> String {
    // orderbook = Orderbook::new(BrokerAsset::BTC, BrokerAsset::USD);
    format!("Hi, {}", "ok")
}

#[marine]
pub fn place_order() -> String {
    let order_asset = parse_asset("BTC").unwrap();
    let price_asset = parse_asset("USD").unwrap();
    
    let order = orders::new_limit_order_request(
        order_asset,
        price_asset,
        OrderSide::Bid,
        0.98,
        5.0,
        SystemTime::now()
    );

    let mut orderbook = get_orderbooks().lock();
    let res = orderbook.process_order(order);
    println!("Processing => {:?}", res);
    format!("Hi, {}", "order placed")
}

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn empty_string(greeting: marine_test_env::greeting::ModuleInterface) {
        let actual = greeting.greeting(String::new());
        assert_eq!(actual, "Hi, ");
    }

    #[marine_test(config_path = "../Config.toml", modules_dir = "../artifacts")]
    fn non_empty_string(greeting: marine_test_env::greeting::ModuleInterface) {
        let actual = greeting.greeting("name".to_string());
        assert_eq!(actual, "Hi, name");
    }
}
