pub type Denomination = Vec<u8>;

pub type LogicHash = u8;

pub type ByteString = Vec<u8>;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Resource {
    pub logic_hash: LogicHash,
    pub static_data: ByteString,
    pub dynamic_data: ByteString,
    pub value: u8,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum ResourceKind {
    Created,
    Consumed,
}

impl Resource {
    pub fn create_resource(
        logic_hash: LogicHash,
        static_data: ByteString,
        dynamic_data: ByteString,
        value: u8,
    ) -> Resource {
        Resource {
            logic_hash,
            static_data,
            dynamic_data,
            value,
        }
    }
    pub fn denomination(&self) -> Denomination {
        [self.static_data.clone(), self.dynamic_data.clone()].concat()
    }
}

pub fn resources_for_denomination(
    denom: Denomination,
    resources: Vec<&Resource>,
) -> Vec<&Resource> {
    resources
        .into_iter()
        .filter(|&x| x.denomination() == denom)
        .collect::<Vec<_>>()
}

pub fn total_quantity_of_denomination(denom: Denomination, resources: Vec<&Resource>) -> u8 {
    resources_for_denomination(denom, resources)
        .iter()
        .map(|&x| x.value)
        .sum()
}
