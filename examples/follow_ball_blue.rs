
// ### DEPRECATED ###


// use fbot_rust_client::{FIRASim, fira_protos};

// const ORIENTATION_KP: f64 = 10.0;
// const ROBOT_SPEED: f64 = 20.0;
// fn main() {
//     let sim = FIRASim::new();

//     loop {
//         let ball = sim.ball();
//         if let Some(goalie) = sim.yellow_robot(&0) {
//             let (target_x, target_y) = (ball.x, ball.y);

//             let diff_x = target_x - goalie.x;
//             let diff_y = target_y - goalie.y;

//             let target_orientation = (diff_y/diff_x).atan();

//             let err_orientation = target_orientation - goalie.orientation;


//             let dist = (diff_x*diff_x + diff_y*diff_y).sqrt();


//             let (vel_left, vel_right) = if dist < 0.1 {
//                 (0.0, 0.0)
//             } else {
//                 let velocidade = err_orientation * ORIENTATION_KP;

//                 if diff_x > 0.0 {
//                     (-velocidade + ROBOT_SPEED, velocidade + ROBOT_SPEED)
//                 } else {
//                     (-velocidade - ROBOT_SPEED, velocidade - ROBOT_SPEED)
//                 }
//             };

//             let commands = fira_protos::Commands {
//                 robot_commands: vec![
//                     fira_protos::Command {
//                         id: 0,
//                         yellowteam: false,
//                         wheel_left: vel_left,
//                         wheel_right: vel_right
//                     },
//                 ]
//             };

//             sim.send_command(commands);
//         }
//     }
// }