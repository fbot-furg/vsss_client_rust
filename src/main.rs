use fbot_rust_client::{Communication};


fn main() {
    let mut a = Communication::new();
    a.start();

    print!("Hello, world!");
}
