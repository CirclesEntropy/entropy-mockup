pub mod resource;
pub mod balance;

fn main() {
    let res = resource::Resource::default();
    println!("{:?}", res.logic_hash);
}
