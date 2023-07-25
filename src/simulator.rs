use crate::balance::*;
use crate::partialtx::{LogicFunction, PartialTx};
use crate::resource::{Denomination, LogicHash, Resource};
use std::collections::HashMap;

pub fn valid_partial_txs(
    map: &HashMap<LogicHash, LogicFunction>,
    ptx: &PartialTx,
) -> Result<Vec<bool>, &'static str> {
    ptx.resources
        .iter()
        .map(|(kind, res)| match map.get(&res.logic_hash) {
            Some(&f) => Ok(f(kind.clone(), ptx.clone())),
            None => Err("couldn't find the function corresponding to this hash"),
        })
        .collect()
}

pub fn is_valid_ptx(
    map: &HashMap<LogicHash, LogicFunction>,
    ptx: &PartialTx,
) -> Result<bool, &'static str> {
    let ptx_validities = valid_partial_txs(map, ptx)?;
    Ok(ptx_validities.iter().all(|x| *x))
}

pub fn make_balance(res: &Resource) -> Balance {
    let mut list: HashMap<Denomination, u8> = HashMap::new();
    list.insert(res.denomination(), res.value);
    Balance(list)
}

pub fn balance_delta(ptx: &PartialTx) -> Balance {
    sub_balances(
        &sum_balances(
            ptx.clone()
                .get_consumed()
                .iter()
                .map(make_balance)
                .collect::<Vec<_>>(),
        ),
        &sum_balances(
            ptx.clone()
                .get_created()
                .iter()
                .map(make_balance)
                .collect::<Vec<_>>(),
        ),
    )
}

pub fn check_transaction(map: HashMap<LogicHash, LogicFunction>, ptxs: Vec<PartialTx>) -> bool {
    let all_valid = ptxs.iter().all(|ptx| is_valid_ptx(&map, ptx).unwrap());
    let zero_sum_deltas = zero_balance(sum_balances(ptxs.iter().map(balance_delta).collect()));
    all_valid && zero_sum_deltas
}
