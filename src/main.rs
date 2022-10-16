use fbot_rust_client::{FIRASim, Referee};

fn main() {
    let sim = FIRASim::new();
    let referee = Referee::new();

    sim.start();
    referee.start();

    loop {
        println!("foul: {:?}", referee.foul());
        println!("ball: {:?}", sim.ball());
    }
}
