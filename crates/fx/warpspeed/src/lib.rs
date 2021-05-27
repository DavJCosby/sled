use colortemp::temp_to_rgb;
use rand::{prelude::ThreadRng, Rng};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Instant,
};

use slc::prelude::*;

const SPAWN_RADIUS: f32 = 0.70;
const MIN_TEMP: i64 = 2100;
const MAX_TEMP: i64 = 6800;

const UPDATE_TIMING: f32 = 1.0 / 500.0;

struct Star {
    start_position: Point,
    position: Point,
    color: Color,
    birthday: Instant,
}

pub struct Warpspeed {
    movement_dir: Vector2D,
    movement_speed: f32,
    stars: Vec<Star>,
    stop: bool,
}

impl Warpspeed {
    pub fn new(movement_dir: Vector2D, movement_speed: f32) -> Self {
        Warpspeed {
            movement_dir,
            movement_speed,
            stars: vec![],
            stop: false,
        }
    }

    fn add_star(&mut self, spawn_center: Point, rng: &mut ThreadRng) {
        let position = (
            spawn_center.0 + (rng.gen::<f32>() - 0.5) * SPAWN_RADIUS,
            spawn_center.1 + (rng.gen::<f32>() - 0.5) * SPAWN_RADIUS,
        );

        let brightness = rng.gen::<f64>() * 0.5;

        let kelvin = rng.gen_range(MIN_TEMP..MAX_TEMP);
        let color64 = temp_to_rgb(kelvin);
        let birthday = Instant::now();

        let star = Star {
            start_position: position,
            position,
            color: (
                (color64.r * brightness) as u8,
                (color64.g * brightness) as u8,
                (color64.b * brightness) as u8,
            ),
            birthday,
        };
        self.stars.push(star);
    }

    fn update_stars(&mut self) {
        let star_vel = (
            -self.movement_dir.0 * self.movement_speed,
            -self.movement_dir.1 * self.movement_speed,
        );
        for mut star in &mut self.stars {
            let elapsed = star.birthday.elapsed().as_secs_f32();
            let new_position = (
                star.start_position.0 + star_vel.0 * elapsed,
                star.start_position.1 + star_vel.1 * elapsed,
            );
            star.position = new_position;
        }
    }

    fn render_stars(&self, controller: &Arc<RwLock<RoomController>>) {
        let mut write = controller.write().unwrap();

        for led in 0..write.room.leds().len() {
            let col = write.room.leds()[led];
            write.set(
                led,
                (
                    (col.0 as f32 * 0.935) as u8,
                    (col.1 as f32 * 0.9425) as u8,
                    (col.2 as f32 * 0.95) as u8, //  artificial blueshift
                ),
            );
        }
        drop(write);

        let read = controller.read().unwrap();

        let view_pos = read.room.view_pos();

        let mut affected_leds: HashMap<usize, (f32, f32, f32)> = HashMap::new();
        for star in &self.stars {
            let dir = (star.position.0 - view_pos.0, star.position.1 - view_pos.1);
            if let Some(id) = read.get_led_at_room_dir(dir) {
                let mut colorf32 = (
                    star.color.0 as f32 / 255.0,
                    star.color.1 as f32 / 255.0,
                    star.color.2 as f32 / 255.0,
                );

                if affected_leds.contains_key(&id) {
                    //println!("overlap");
                    let old = affected_leds[&id];
                    colorf32 = (colorf32.0 + old.0, colorf32.1 + old.1, colorf32.2 + old.2);
                }

                affected_leds.insert(id, colorf32);
            }
        }

        drop(read);

        let mut write = controller.write().unwrap();
        // reinhard tonemapping
        for (id, colorf32) in affected_leds {
            let tonemapped = (
                colorf32.0 / (colorf32.0 + 1.0),
                colorf32.1 / (colorf32.1 + 1.0),
                colorf32.2 / (colorf32.2 + 1.0),
            );

            let tonemappedu8 = (
                (tonemapped.0 * 255.0) as u8,
                (tonemapped.1 * 255.0) as u8,
                (tonemapped.2 * 255.0) as u8,
            );
            write.set(id, tonemappedu8);
        }

        drop(write);
    }
}

impl InputDevice for Warpspeed {
    fn start(mut self, controller: Arc<RwLock<RoomController>>) {
        thread::spawn(move || {
            let read = controller.read().unwrap();
            let spawn_center = (
                read.room.view_pos().0 + self.movement_dir.0 * 5.0,
                read.room.view_pos().1 + self.movement_dir.1 * 5.0,
            );
            drop(read);

            let mut rng = rand::thread_rng();

            let start = Instant::now();
            let mut last = 0.0;
            let mut next_spawn = 0.0;

            while !self.stop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                }

                if duration > next_spawn {
                    self.add_star(spawn_center, &mut rng);
                    next_spawn = duration + rng.gen::<f32>() * 0.225;
                }

                self.stars
                    .retain(|star| star.birthday.elapsed().as_secs_f32() < 20.0);

                self.update_stars();
                self.render_stars(&controller);
                last = duration;
            }
        });
    }

    fn stop(&mut self) {
        self.stop = true;
    }
}
