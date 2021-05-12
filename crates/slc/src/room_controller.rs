use std::sync::{Arc, RwLock};

use crate::{room::Room, util::*};

/// Contains methods for reading and writing room data.
/// Upon construction, comsumes the [Room](../room/struct.Room.html).
/// Should be packed into a [RwLock](std::sync::RwLock) using [new_thread_safe()](#method.new_thread_safe).
/// The RwLock's write lock should only be obtained by an [InputDevice](../devices/trait.InputDevice.html).
pub struct RoomController {
    pub room: Room,
}

impl RoomController {
    /// Creates a RoomController by consuming room, and then wrap the RoomController for thread safety.
    pub fn new_thread_safe(room: Room) -> Arc<RwLock<RoomController>> {
        Arc::new(RwLock::new(RoomController { room }))
    }

    /// Sets the color of a given led
    pub fn set_led(&mut self, index: usize, color: Color) {
        self.room.leds[index] = color;
    }
    /// Casts a ray in the given direction. If it hits a wall, the led closest to that wall position will be colored.
    pub fn set_led_at_dir(&mut self, dir: Vector2D, color: Color) {
        let view_pos = self.room.view_pos;
        let dist = 100.0;
        let ray_end = (view_pos.0 + (dir.0 * dist), view_pos.1 + (dir.1 * dist));
        let mut intersection: Option<Point> = None;
        let mut strip_index = 0;
        let mut led_count = 0.0;

        for strip in &self.room.strips {
            let i = strip.intersects(&(view_pos, ray_end));
            if i.is_some() {
                intersection = i;
                break;
            }
            strip_index += 1;
            led_count += strip.len() * self.room.led_density;
        }

        if intersection.is_none() {
            return;
        }

        let strip = self.room.strips[strip_index];
        let intersection_point = intersection.unwrap();
        let tx = reverse_lerp(strip.0, strip.1, intersection_point);
        led_count += tx * self.room.led_density * strip.len();
        if led_count > 0.0 {
            led_count -= 1.0;
        }
        self.set_led(led_count as usize, color);
    }
}

/// if lerp(a, b, t) = c, reverse_lerb(a, b, c) = t
fn reverse_lerp(a: Point, b: Point, c: Point) -> f32 {
    if a.0 != b.0 {
        (c.0 - a.0) / (b.0 - a.0)
    } else {
        (c.1 - a.1) / (b.1 - a.1)
    }
}
