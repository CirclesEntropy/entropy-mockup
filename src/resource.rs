pub type LogicHash = u8;

pub type ByteString = u8;

pub type Denomination = (LogicHash, ByteString);

#[derive(Clone, Copy,  Debug, PartialEq, PartialOrd)]
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
    pub fn denomination(self) -> Denomination {
     (self.logic_hash, self.static_data)
    }
     pub fn set_value(self, value: u8) -> Self {
        Resource::create_resource(self.logic_hash, self.static_data, self.dynamic_data, value)
     }
}

    pub fn quantity_of_denomination(denom: Denomination, resources: &Vec<Resource>) -> u8
    {
   resources
        .into_iter()
        .filter(|&x| x.denomination() == denom)
        .map(|x| x.value)
        .sum()
}
