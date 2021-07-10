use rftracer::rftracer;

#[rftracer]
fn hello() {
    println!("Hello world!");
}

#[test]
fn hello_test() {
    hello();
    assert_eq!(0, 0);
}
