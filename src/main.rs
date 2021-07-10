use rftracer::rftracer;

#[rftracer]
fn hello() {
    println!("Hello world!");
}

fn main() {
    hello();
}
