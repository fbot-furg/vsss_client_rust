use vsss_rust_client::{SSLVISION};

fn main() {
    loop {
        let my_robot = SSLVISION.blue_robot(&0);
        let enemy_robot = SSLVISION.yellow_robot(&0);

        let ball = SSLVISION.ball();

        println!("SSL_VISION Robot: {:?}", my_robot);
        println!("SSL_VISION Enemy Robot: {:?}", enemy_robot);
        println!("SSL_VISION Ball: {:?}", ball);
    }
}
