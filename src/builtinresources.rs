use crate::resource:: Resource;


pub mod always_valid {
    use crate::partialtx::LogicFunction;
    use crate::resource::LogicHash;
    
    pub const LOGIC_FUNCTION: LogicFunction =  |_, _| true;
    pub const LOGIC_HASH : LogicHash = 0;
}

const DUMMY_RESOURCE: Resource =  Resource {
            logic_hash: always_valid::LOGIC_HASH,
            static_data: vec![],
            dynamic_data: vec![],
            value: 0
        };

