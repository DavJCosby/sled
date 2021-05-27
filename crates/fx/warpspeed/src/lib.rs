use colortemp::temp_to_rgb;
use rand::{prelude::ThreadRng, Rng};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Instant,
};

use slc::prelude::*;

const SPAWN_RADIUS: f32 = 0.25;
const MIN_TEMP: i64 = 2100;
const MAX_TEMP: i64 = 6800;

const UPDATE_TIMING: f32 = 1.0 / 144.0;

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

    fn add_star(&mut self, spawn_center: Point) {
        let mut rng = rand::thread_rng();
        let position = (
            spawn_center.0 + (rng.gen::<f32>() - 0.5) * SPAWN_RADIUS,
            spawn_center.1 + (rng.gen::<f32>() - 0.5) * SPAWN_RADIUS,
        );

        let kelvin = rng.gen_range(MIN_TEMP..MAX_TEMP);
        let color64 = temp_to_rgb(kelvin);
        let birthday = Instant::now();

        let star = Star {
            start_position: position,
            position,
            color: (
                (color64.r / 10.0) as u8,
                (color64.g / 10.0) as u8,
                (color64.b / 10.0) as u8,
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
        write.set_all((0, 0, 0));
        let view_pos = write.room.view_pos();

        let mut affected_leds: HashMap<usize, (f32, f32, f32)> = HashMap::new();
        for star in &self.stars {
            let dir = (star.position.0 - view_pos.0, star.position.1 - view_pos.1);
            if let Some(id) = write.get_led_at_room_dir(dir) {
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
                read.room.view_pos().0 + self.movement_dir.0 * 6.0,
                read.room.view_pos().1 + self.movement_dir.1 * 6.0,
            );
            drop(read);

            let start = Instant::now();
            let mut last = 0.0;

            let mut next_spawn = 0.0;

            while !self.stop {
                let duration = start.elapsed().as_secs_f32();
                if duration - last < UPDATE_TIMING {
                    continue;
                }

                if duration > next_spawn {
                    self.add_star(spawn_center);
                    next_spawn = duration + rand::thread_rng().gen::<f32>() * 0.13;
                }

                for star in &self.stars {
                    if star.birthday.elapsed().as_secs_f32() > 10.0 {
                        drop(star);
                    }
                }

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
