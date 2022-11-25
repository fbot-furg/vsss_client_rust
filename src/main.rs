use vsss_rust_client::{SSLVISION};

fn main() {
    loop {
        println!("SSLVISION Robot: {:?}", SSLVISION.blue_robot(&0));
        // println!("SSLVISION ball: {:?}", SSLVISION.ball());
    }
}
