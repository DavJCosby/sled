# SLC - Spatial Lighting Controller
Spatial Lighting Controller is a project I've started to give myself advanced control over the
individually-addressable LED strip lights in my room. What separates SLC (pronounced silk) from other LED controller is that it allows you to map out the user's room in 2D space, imagine a camera within that room, and light up LEDs using that camera's coordinate space.

```rs
use slc::prelude::*;

let room = Room::new("path/to/config.rcfg");
let room_controller = RoomController::new(room);

// set all pixels white
room_controller.set_all((255, 255, 255));
// set the LED leftmost to the camera red
room_controller.set_at_camera_dir((-1.0, 0.0), (255, 0, 0));
// set the LED rightmost to the camera green
room_controller.set_at_camera_angle(PI * -0.5, (0, 255, 0));
// set the northmost LED blue (relative to the room's coordinate space)
room_controller.set_at_room_dir((0.0, 1.0), (0, 0, 255));

// --connect room_controller to an output device--
```
#
## *Disclaimer*
The SLC crate is not in charge of interfacing with GPIO pins or activating any real-world LEDs; you will need to write your own OutputDevice to handle that. SLC is merely a set of tools that lets you think of your LEDs in spatial terms. Below are some example to get you started building your own Input and Output devices.

* [Sweep](crates/slc_sweep) - A simple visual effect that sweeps a green light around the room, kinda like a radar.
* [Gui](crates/slc_gui) - 3D Gui powered by Bevy that lets you preview your room and visual effects.

# Basic API

While the above code example technically works, your typical project is more likely to look something like this:

```rs
pub fn main() {
    let room = Room::new("path/to/config.rcfg");
    // create a RoomController with a RwLock for safe multithreading
    let rc_input_handle = RoomController::new_thread_safe(room);
    let rc_output_handle = rc_input_handle.clone();
    // prepare input and output devices
    let input = MyInputDevice::new();
    let output = MyOutputDevice::new();

    input.start(rc_input_handle);
    output.start(rc_output_handle);
}
```
## IO
Input and Output Devices should implement the traits [InputDevice and OutputDevice](crates/slc/src/devices.rs), respectively. Since we have created thread-safe input and output handles in the example above, you are encouraged to run continuous code in the `.start()` methods on a separate thread.
 
OutputDevices should only access the RoomController in read-only fashion. The RwLock given by `::new_thread_safe()` lets us enforce that rule.

### `MyInputDevice::start()`
```rs
// --snip--
let mut controller_write_access = rc_input_handle.write().unwrap();

controller_write_access.set_LED(0, (0, 255, 0));
drop(controller_write_access);
```

### `MyOutputDevice::start()`
```rs
// --snip--
let mut controller_read_access = rc_output_handle.read().unwrap();

println!("Our light strips have a density of {} LEDs/meter.", controller_read_access.room.density);
drop(controller_write_access);
```

InputDevices are required to inplement a `.stop()` method, while OutputDevices do not. This is because you likely will want to swap out visual effects at run-time, but you are unlikely to need to swap out display methods on-the-go.

## Room and RoomController

A Room is typically created from a room configuration (.rcfg) file, which follows the format below:
```rs
// SUBJECT TO CHANGE
60                      // LED density (LEDs/meter)
0.75 0.75               // Position (meters) of the "Camera"
90                      // Rotation (degrees) of the "Camera" (0 = facing right, 90 = facing up)
0 -1.5 2.0 0            // strip0_start.x strip0_start.y strip0_end.x strip0_end.y
2.0 0 1.5 1.5           // strip1_start.x strip1_start.y strip1_end.x strip1_end.y
1.5 1.5 1.5 2.0         // strip2_start.x strip2_start.y strip0_end.x strip2_end.y
                        // and so on...
```

Running `room::new(filepath)` will parse through this data to create a room. Using the density and strip coordinates provided, it calculates the number of LEDs in the room and uses that number to create a vector (u8, u8, u8) tuples. Each tuple represents an LED's color in the room.

The constructor `RoomController:new(room)` takes ownership of the room and sets up some helper methods for interacting with the LEDs spatially. All data in the room is still accessible, so you can do stuff like this:

```
println!("room has {} strips.", room_controller.room.strips.len());
```

For more information, check out the Docs (NOT DONE YET).

# Extra
## ToDo
- Implement the new RoomController API
- Docs

## BRAIN DUMP
- could also rename to SLED (spatial LED)