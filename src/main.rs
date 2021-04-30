use room::*;
use std::env;

use crate::room_controller::RoomController;
mod room;
mod room_controller;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    //let w1 = ((1.5, 0.0), (1.25, 1.5));
    //let w2 = ((0.75, 0.75), (2.0, 1.75));
    //println!("intersection: {:?}", w1.intersects(&w2));

    println!("{}", args.get(1).unwrap());

    let room = Room::new_from_file(args.get(1).expect("no argument given for filename."));
    let mut controller = RoomController { room };
    println!("{:?}", controller.room.strips);

    controller.set_led_at_dir((1.0, 0.0), (1, 1, 1));

    for led in controller.room.leds {
        println!("\t{:?}", led);
    }
}
