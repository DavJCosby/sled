use std::{
    cmp::Ordering,
    f32::consts::{PI, TAU},
    sync::{Arc, RwLock},
};

use crate::{room::Room, util::*};

/// Contains methods for reading and writing room data.
/// Upon construction, comsumes the [Room](../room/struct.Room.html).
/// Should be packed into a [RwLock](std::sync::RwLock) using [new_thread_safe()](#method.new_thread_safe).
/// The RwLock's write lock should only be obtained by an [InputDevice](../devices/trait.InputDevice.html).
pub struct RoomController {
    pub room: Room,
    angle_dir_led_index_triplets: Vec<(f32, Vector2D, usize)>,
}

impl RoomController {
    pub fn new(room: Room) -> Self {
        let mut angle_dir_led_index_triplets: Vec<(f32, Vector2D, usize)> = vec![];

        let max = room.leds().len();
        let view = room.view_pos();

        for index in 0..max {
            let t = index as f32 / max as f32;
            let p = room.get_pos_at_t(t);
            let d = (p.0 - view.0, p.1 - view.1);
            let angle = d.1.atan2(d.0);
            angle_dir_led_index_triplets.push((
                (angle + TAU) % TAU,
                (angle.cos(), angle.sin()),
                index,
            ));
        }

        // sort by angle to make ranges faster
        angle_dir_led_index_triplets.sort_by(|p0, p1| {
            if p0.0 < p1.0 {
                Ordering::Less
            } else if p0.0 > p1.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        //println!("{:?}", angle_dir_led_index_triplets);

        RoomController {
            room,
            angle_dir_led_index_triplets,
        }
    }

    /// Creates a RoomController by consuming room, and then wrap the RoomController for thread safety.
    pub fn new_thread_safe(room: Room) -> Arc<RwLock<RoomController>> {
        let rc = RoomController::new(room);
        Arc::new(RwLock::new(rc))
    }

    /// Sets the color of a given led
    pub fn set(&mut self, index: usize, color: Color) {
        self.room.set_led(index, color);
    }

    /// sets the color of all leds in the room
    pub fn set_all(&mut self, color: Color) {
        for index in 0..self.room.leds().len() {
            self.room.set_led(index, color);
        }
    }

    pub fn set_at_view_dir(&mut self, dir: Vector2D, color: Color) {
        self.set_at_view_angle(dir.1.atan2(dir.0) - (PI / 2.0), color);
    }

    pub fn set_at_view_angle(&mut self, angle: f32, color: Color) {
        let room_angle = self.room.view_rot() + angle;
        self.set_at_room_angle(room_angle, color);
    }

    pub fn set_at_room_angle(&mut self, angle: f32, color: Color) {
        let adjusted = angle + (PI / 2.0);
        let room_dir = (adjusted.cos(), adjusted.sin());
        self.set_at_room_dir(room_dir, color);
    }

    /// Casts a ray in the given direction, in room coordinate space, from the camera's position.
    /// If it hits a wall, the led closest to that wall position will be colored.
    pub fn set_at_room_dir(&mut self, dir: Vector2D, color: Color) {
        let view_pos = self.room.view_pos();
        let dist = 100.0;
        let ray_end = (view_pos.0 + (dir.0 * dist), view_pos.1 + (dir.1 * dist));
        let mut intersection: Option<Point> = None;
        let mut strip_index = 0;
        let mut led_count = 0.0;

        for strip in self.room.strips() {
            let i = strip.intersects(&(view_pos, ray_end));
            if i.is_some() {
                intersection = i;
                break;
            }
            strip_index += 1;
            led_count += strip.len() * self.room.density();
        }

        if intersection.is_none() {
            return;
        }

        let strip = self.room.strips()[strip_index];
        let intersection_point = intersection.unwrap();
        let tx = reverse_lerp(strip.0, strip.1, intersection_point);
        led_count += tx * self.room.density() * strip.len();
        if led_count > 0.0 {
            led_count -= 1.0;
        }
        self.set(led_count as usize, color);
    }

    pub fn map_angle_to_color(&mut self, map: &dyn Fn(f32) -> Color) {
        for (angle, _dir, led_index) in &self.angle_dir_led_index_triplets {
            let color = map(*angle);
            self.room.set_led(*led_index, color);
        }
    }

    pub fn map_angle_to_color_clamped(
        &mut self,
        map: &dyn Fn(f32) -> Color,
        min_angle: f32,
        max_angle: f32,
    ) {
        let adjusted_min = (min_angle + TAU) % TAU;
        let adjusted_max = (max_angle + TAU) % TAU;
        let crosses_wraparound = min_angle < 0.0 && max_angle > 0.0;

        for (angle, _dir, led_index) in &self.angle_dir_led_index_triplets {
            let deref_angle = *angle;

            // if this angle doesn't fit in the arc, skip it
            if crosses_wraparound {
                if !((deref_angle < TAU && deref_angle > adjusted_min)
                    || (deref_angle > 0.0 && deref_angle < adjusted_max))
                {
                    continue;
                }
            } else if !(deref_angle > adjusted_min && deref_angle < adjusted_max) {
                continue;
            }

            self.room.set_led(*led_index, map(deref_angle));
        }
    }

    pub fn map_dir_to_color(&mut self, map: &dyn Fn(Vector2D) -> Color) {
        for (_angle, dir, led_index) in &self.angle_dir_led_index_triplets {
            let color = map(*dir);
            self.room.set_led(*led_index, color);
        }
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
