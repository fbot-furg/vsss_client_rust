use fbot_rust_client::{FIRASim, Referee};

fn main() {
    let sim = FIRASim::new();
    let referee = Referee::new();

    loop {
        let ball = sim.ball();
        let referee = referee.referee();

        println!("Ball: {:?}", ball);
        println!("Referee: {:?}", referee);
    }
}
