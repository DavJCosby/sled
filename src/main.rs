use room::*;
use std::env;
mod room;
mod room_controller;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let w1 = ((0.0, 0.0), (1.5, 0.0));
    let w2 = ((1.5, 0.0), (1.5, 1.5));
    let w3 = ((1.5, 1.5), (0.0, 1.5));

    //let config = RoomConfig::new(60.0, (0.0, 0.0), 90.0);

    println!("{}", args.get(1).unwrap());

    let config2 = RoomConfig::new_from_file(args.get(1).expect("no argument given for filename."));

    println!("{}", config2.num_leds());
}
