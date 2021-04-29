use room::*;

mod room;
mod room_controller;

pub fn main() {
    let w = Strip(Point(0.0, 0.0), Point(1.5, 0.0));
    let w2 = Strip(Point(1.5, 0.0), Point(1.5, 1.5));
    let w3 = Strip(Point(1.5, 1.5), Point(0.0, 1.5));

    let config = RoomConfig {
        led_density: 60.0,
        view_pos: Point(0.0, 0.0),
        view_rot: 90.0,
        strips: vec![w, w2, w3],
        leds: vec![Color(0, 0, 0)],
    };

    println!("{}", config.num_leds());
}
