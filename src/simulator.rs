use crate::resource::{LogicHash, Resource, Denomination};
use crate::partialtx::{PartialTx, LogicFunction};
use crate::balance::*;
use std::collections::HashMap;

pub fn valid_partial_txs (map: &HashMap<LogicHash, LogicFunction>, ptx: &PartialTx) -> Result<Vec<bool>, &'static str>
{
    ptx.resources.iter().map(|(kind, res)|
                                            match  map.get(&res.logic_hash) {
                                                Some(&f) => Ok(f(kind, ptx)),
                                                None =>  return Err("couldn't find the function corresponding to this hash")}).collect()
}

pub fn is_valid_ptx (map: &HashMap<LogicHash, LogicFunction>, ptx: &PartialTx) -> Result<bool, &'static str>{
    let ptx_validities =valid_partial_txs(map, ptx)?;
    Ok(ptx_validities.iter().all(|x| *x))
}

pub fn make_balance(res: &Resource) -> Balance {
    let mut list:HashMap<Denomination, u8> = HashMap::new();
    list.insert(res.denomination(), res.value);
    Balance(list)
}

pub fn balance_delta (ptx: &PartialTx) -> Balance {
    sub_balances(&sum_balances(ptx.clone().get_consumed()
                                  .iter()
                                  .map(|x| make_balance(x))
                                  .collect::<Vec<_>>()),
                                  &sum_balances(ptx.clone().get_created()
                                               .iter()
                                               .map(|x| make_balance(x))
                                               .collect::<Vec<_>>()))
}

pub fn check_transaction(map: &HashMap<LogicHash, LogicFunction> , ptxs:  Vec<&PartialTx>) -> bool {
    
    let all_valid = ptxs.iter().all(|ptx| is_valid_ptx(map, ptx).unwrap());
    let zero_sum_deltas = zero_balance(sum_balances(ptxs.iter().map(|x|balance_delta(x)).collect()));
    all_valid && zero_sum_deltas
}