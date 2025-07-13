use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    //HashMap::from([(address, amount)])
    let mut balances = HashMap::new();
    balances.insert(address, amount);
    balances
}
