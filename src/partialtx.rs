use crate::resource::{Resource, ResourceKind};

pub type LogicFunction = fn(&ResourceKind, &PartialTx) -> bool;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct PartialTx {
    pub resources: Vec<(ResourceKind, Resource)>,
}

impl PartialTx {
    pub fn get_created(self: PartialTx) -> Vec<Resource> {
        self.resources
            .into_iter()
            .filter_map(|(x, y)| match x {
                ResourceKind::Created => Some(y),
                _ => None,
            })
            .collect()
    }
    pub fn get_consumed(self: PartialTx) -> Vec<Resource> {
        self.resources
            .into_iter()
            .filter_map(|(x, y)| match x {
                ResourceKind::Created => Some(y),
                _ => None,
            })
            .collect()
    }
}
