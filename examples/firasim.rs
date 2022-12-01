use vsss_rust_client::{FIRASIM};

fn main() {
    loop {
        let my_robot = FIRASIM.blue_robot(&0);
        let enemy_robot = FIRASIM.yellow_robot(&0);

        let ball = FIRASIM.ball();

        println!("Robot x:{} y:{}", my_robot.x, my_robot.y);
        println!("Enemy Robot: x:{} y:{}", enemy_robot.x, enemy_robot.y);
        println!("Ball: x:{} y:{}", ball.x, ball.y);
    }
}
