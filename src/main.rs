use fbot_rust_client::{FIRASIM, REFEREE};

fn main() {
    loop {
        println!("FIRASIM: {:?}", FIRASIM.ball());
        println!("REFEREE: {:?}", REFEREE.foul());
    }
}
