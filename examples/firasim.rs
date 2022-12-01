use vsss_rust_client::{FIRASIM};

fn main() {
    loop {
        let my_robot = FIRASIM.blue_robot(&0);
        let enemy_robot = FIRASIM.yellow_robot(&0);

        let ball = FIRASIM.ball();

        println!("FIRASIM Robot: {:?}", my_robot);
        println!("FIRASIM Enemy Robot: {:?}", enemy_robot);
        println!("FIRASIM Ball: {:?}", ball);
    }
}
