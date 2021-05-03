use crate::{room::Room, util::*};

pub struct RoomController {
    pub room: Room,
}

impl RoomController {
    /// sets the color of a given led
    pub fn set_led(&mut self, index: usize, color: Color) {
        self.room.leds[index] = color;
    }

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
