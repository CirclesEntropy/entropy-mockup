use rand::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub  struct Denomination (Vec<u8>);

#[derive(Clone, Debug, Copy)]
struct LogicHash (u8);

#[derive(Clone, Debug)]
struct ByteString (Vec<u8>);

#[derive (Clone, Debug)]
pub struct Resource {
    logic_hash : LogicHash,
    static_data: ByteString,
    dynamic_data: ByteString,
    value: u8
}

impl Default for Resource {
    fn default () -> Self {
       Resource{ logic_hash: LogicHash (random()),
        static_data: ByteString(vec![]),
        dynamic_data: ByteString(vec![]),
                 value: 0}
    }
}
impl Resource {
    pub fn denomination (self: &Self) -> Denomination
    {
        Denomination ([self.static_data.0.clone(), self.dynamic_data.0.clone()].concat())
    }
}

impl Denomination {
    pub fn resources_for_denomination (denom: Denomination, resources: Vec<Resource>) -> Vec<Resource>
    {
        resources.into_iter().filter(|x| x.denomination() == denom).collect::<Vec<_>>()
    }

    pub fn total_quantity_of_denomination (denom: Denomination, resources: Vec<Resource>) -> u8
    {
        Self::resources_for_denomination(denom, resources).iter()
            .map(|x| x.value)
            .fold(0, |acc, x| acc + x)
    }
}




