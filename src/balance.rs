use crate::resource::Denomination;

use std::collections::HashMap;

pub struct Balance(pub HashMap<Denomination, u8>);

fn negate_balance(balance: &Balance) -> Balance {
    let mut negated: HashMap<Denomination, u8> = HashMap::new();
    let _ = balance
        .0
        .iter()
        .map(|(x, y)| negated.insert(x.to_vec(), 0 - y));
    Balance(negated)
}

pub fn add_balances(balance0: &Balance, balance1: &Balance) -> Balance {
    let mut new_balances: HashMap<Denomination, u8> = HashMap::new();

    let _ = balance1.0.iter().map(|(k, &v2)| match balance0.0.get(k) {
        Some(&v1) => new_balances.insert(k.to_vec(), v1 + v2),
        None => new_balances.insert(k.to_vec(), v2),
    });
    Balance(new_balances)
}

pub fn sub_balances(balance0: &Balance, balance1: &Balance) -> Balance {
    let negated = negate_balance(balance1);
    add_balances(balance0, &negated)
}

pub fn sum_balances(balances: Vec<Balance>) -> Balance {
    let empty_balance = Balance(HashMap::new());
    balances
        .iter()
        .fold(empty_balance, |acc, b| add_balances(&acc, b))
}

pub fn zero_balance(balance: Balance) -> bool {
    balance.0.iter().all(|(_, &v)| v == 0)
}
