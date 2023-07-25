// use crate::resource::Resource;

use crate::partialtx::LogicFunction;
use crate::resource::LogicHash;
use std::collections::HashMap;

pub const LOGIC_FUNCTION: LogicFunction = |_, _| true;
pub const LOGIC_HASH: LogicHash = 0;

pub fn make_logic_fn_map(
    mut map: HashMap<LogicHash, LogicFunction>,
) -> HashMap<LogicHash, LogicFunction> {
    map.insert(LOGIC_HASH, LOGIC_FUNCTION);
    map
}

//   pub static DUMMY_RESOURCE: Resource = Resource {
//   logic_hash: always_valid::LOGIC_HASH,
//   static_data: vec![],
//   dynamic_data: vec![],
//   value: 0,
//   };
