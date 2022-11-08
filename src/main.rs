use fbot_rust_client::{SSLVISION};

fn main() {
    loop {
        println!("SSLVISION ball: {:?}", SSLVISION.ball());
    }
}
