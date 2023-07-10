// define resources
use crate::resource::Resource;
use std::collections::HashMap;

fn init_resources() -> HashMap<&'static str, Resource> {
    let mut resources = HashMap::new();

    resources.insert("dolphin", Resource::create_resource(0, vec![0], vec![0], 0));
    resources.insert(
        "starfish",
        Resource::create_resource(0, vec![1], vec![0], 0),
    );
    resources.insert("octopus", Resource::create_resource(0, vec![1], vec![0], 0));
    resources
}
