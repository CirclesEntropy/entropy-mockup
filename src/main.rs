pub mod balance;
pub mod builtinresources;
pub mod partialtx;
pub mod resource;
pub mod simulator;

use crate::partialtx::{LogicFunction, PartialTx};
use crate::resource::{quantity_of_denomination, Denomination, LogicHash, Resource, ResourceKind};
use crate::simulator::*;
use std::collections::HashMap;

fn main() {
    let txs: Vec<PartialTx> = vec![alice::ptx(), bob::ptx(), solver::ptx()];
    let logics = init_logicfns();
//     let ptxp = txs
//         .iter()
//         .map(|x| simulator::is_valid_ptx(&logics, x))
//         .collect::<Vec<_>>();
//     println!("{:?}", ptxp);
    assert!(check_transaction(logics, txs), "2pex transactions invalid");
}

pub fn dolphin() -> Resource {
    Resource::create_resource(0, 0, 0, 0)
}

pub fn octopus() -> Resource {
    Resource::create_resource(0, 1, 0, 0)
}

pub fn starfish() -> Resource {
    Resource::create_resource(0, 2, 0, 0)
}

fn init_logicfns() -> HashMap<LogicHash, LogicFunction> {
    let mut fns = HashMap::new();
    let alice_logic: LogicFunction = alice_intent_logic;
    fns.insert(1, alice_logic);
    builtinresources::make_logic_fn_map(fns)
}

pub fn alice_intent_logic(kind: ResourceKind, tx: PartialTx) -> bool {
    let created = tx.get_created();
    let dolphin: Denomination = dolphin().denomination();
    let octopus: Denomination = octopus().denomination();
    let starfish: Denomination = starfish().denomination();
    kind == ResourceKind::Created
        || quantity_of_denomination(dolphin, &created) == 1
            && (quantity_of_denomination(octopus, &created) == 1
                || quantity_of_denomination(starfish, &created) == 2)
}

pub fn alice_intent() -> Resource {
    Resource::create_resource(1, 3, 0, 0)
}

mod alice {
    use super::*;
    use crate::partialtx::PartialTx;
    use crate::resource::ResourceKind;

    pub fn ptx() -> PartialTx {
        PartialTx {
            resources: vec![
                (ResourceKind::Created, alice_intent().set_value(1)),
                (ResourceKind::Consumed, octopus().set_value(1)),
                (ResourceKind::Consumed, starfish().set_value(1)),
            ],
        }
    }
}

mod bob {
    use super::*;
    use crate::partialtx::PartialTx;
    use crate::resource::ResourceKind;

    pub fn ptx() -> PartialTx {
        PartialTx {
            resources: vec![
                (ResourceKind::Created, octopus().set_value(1)),
                (ResourceKind::Consumed, dolphin().set_value(1)),
            ],
        }
    }
}

mod solver {
    use super::*;
    use crate::partialtx::PartialTx;
    use crate::resource::ResourceKind;

    pub fn ptx() -> PartialTx {
        PartialTx {
            resources: vec![
                (ResourceKind::Consumed, alice_intent().set_value(1)),
                (ResourceKind::Created, dolphin().set_value(1)),
                (ResourceKind::Created, starfish().set_value(2)),
            ],
        }
    }
}
