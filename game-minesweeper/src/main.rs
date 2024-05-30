use playground::Playground;

pub mod colors;
pub mod config;
pub mod playground;

fn main() {
    println!("Hello, world!");
    let mut playground = Playground::new(10, 10);
    playground.create_playground();
}
