# SLC - Spatial Lighting Controller
```toml
slc = { git = "https://github.com/DavidCosbyUofU/slc" }
```

Spatial Lighting Controller is a project I've started to give myself advanced control over the
individually-addressable LED strip lights in my room. What separates SLC (pronounced silk) from other LED controller is that it allows you to map out the user's room in 2D space, imagine a camera within that room, and light up LEDs using that camera's coordinate space.

```rs
use slc::prelude::*;

let room = Room::new("path/to/config.rcfg");
room.set_input_device(CustomInput);
room.add_output_device(CustomOutput);
room.start();

struct CustomInput;
struct CustomOutput;

impl InputDevice for CustomInput {
    fn start(&self, input_handle: RoomControllerInputHandle) {
        let mut room_controller = input_handle.write().unwrap();
        // set all pixels white
        room_controller.set_all((255, 255, 255));
        // set the LED leftmost to the camera red
        controller_write.set_at_view_angle(PI, (255, 0, 0));
        // set the LED rightmost to the camera green
        controller_write.set_at_view_dir((1.0, 0.0), (0, 255, 0));
        // set the northmost LED blue (relative to the room's coordinate space)
        room_controller.set_at_room_dir((0.0, 1.0), (0, 0, 255));
    }

    fn stop(&mut self) {}
}

impl OutputDevice for CustomOutput {
    fn start(&self, output_handle: RoomControllerOutputHandle) {
        let room_controller = output_handle.read().unwrap();

        for led in room_controller.room_data.leds() {
            println!("{:?}", led);
        }
    }
}
```
#
## *Disclaimer*
The SLC crate is not in charge of interfacing with GPIO pins or activating any real-world LEDs; you will need to write your own OutputDevice to handle that. SLC is merely a set of tools that lets you think of your LEDs in spatial terms. 

In my setup, my PC does all the calculations for InputDevices and then streams the LEDS over to my Raspberry Pi via a TCP OutputDevice. The Pi listens to that TCP connection via an InputDevice, and then displays them on the real LEDs via a GPIO OutputDevice. See [this repository](https://github.com/DavidCosbyUofU/slc_tcp) for more information. 

# Basic API
All mentioned features have documentation, which you can read [here]((http://davidcosbyuofu.github.io/doc/slc)).

## IO
Input and Output Devices should implement the traits [InputDevice and OutputDevice](src/devices.rs), respectively. Since we have created thread-safe input and output handles in the example above, you are encouraged to run continuous code in the `.start()` methods on a separate thread.
 
OutputDevices should only access the RoomController in read-only fashion. The RoomControllerOutputHandle passed in through the start method is actually just a RwLock around a RoomController, which helps us keep our mutability straight.
### `MyInputDevice.start()`
```rs
// --snip--
let mut controller_write_access = input_handle.write().unwrap();
println!("Our LED strips have a density of {} LEDs/meter.", controller_write_access.room_data.density); // also okay

controller_write_access.set(0, (0, 255, 0)); // okay
```

### `MyOutputDevice.start()`
```rs
// --snip--
let mut controller_read_access = input_handle.read().unwrap();

println!("Color of the 1st LED: {:?}", controller_read_access.room_data.leds()[1]); // okay
controller_read_access.set(0, (0, 255, 0)); // illegal, will not compile
```

InputDevices are required to inplement a `.stop()` method, while OutputDevices do not. This is because you likely will want to swap out visual effects at run-time, but you are unlikely to need to swap out display methods on-the-go.

## Room, RoomController, and RoomData
Constructing a Room creates a RoomData struct behind the scenes, which can be accessed via a RoomController. Rooms are created from a room configuration (.rcfg) file, which follows the format below:
```rs
60                      // LED density (LEDs/meter)
0.75 0.75               // Position (meters) of the "Camera"
0                       // Rotational offset (degrees, counter-clockwise, auto converted into radians) of the "Camera" (0 = facing right, 90 = facing up)
0 -1.5 2.0 0            // strip0_start.x strip0_start.y strip0_end.x strip0_end.y
2.0 0 1.5 1.5           // strip1_start.x strip1_start.y strip1_end.x strip1_end.y
1.5 1.5 1.5 2.0         // strip2_start.x strip2_start.y strip0_end.x strip2_end.y
                        // and so on...
```

Running `Room::new(filepath)` will parse through this data to create RoomData. Using the density and strip coordinates provided, it calculates the number of LEDs in the room and uses that number to create a vector of (u8, u8, u8) tuples. Each tuple represents an LED's color in the room.

Rooms can be given an InputDevice and multiple OutputDevices via its `set_input_device()` and `add_output_device()` methods. Calling `.start()` on a Room will run the `.start()` method on each connected Input and Output devices. Each device receives either a RoomControllerInputHandle or a RoomControllerOutputHandle, a thread-safe wrapper equivalent to `Arc<RwLock<RoomController>>`.

## Mapping
The "set at view/direction" methods showcased earlier are good for when you need a color at a precise
direction, but because they rely on a ray-line segment intersection algorithm to find the target led, they can be expensive. SLC offers an alternative approach to directional coloring, in the form of maps.

```rs
// --snip--
let uv_map = |(dx, dy): (f32, f32)| {
    let r = ((dx + 1.0) * 0.5) * 255.0;
    let g = ((dy + 1.0) * 0.5) * 255.0;
    (r as u8, g as u8, 0)
};

room_controller.map_dir_to_color(&uv_map);
```

If you only want to color the leds within a certain area, you can set a `min_angle` and a `max_angle`.

```rs
let map = |(dx, dy): (f32, f32)| { ... };
room_controller.map_dir_to_color_clamped(&map, 0.0, PI / 4.0);
```

See the [docs](http://davidcosbyuofu.github.io/doc/slc/room_controller) for a full list of mapping methods.

# Examples
Below are some examples to get you started building your own input and output devices.

### Input
* [Calibration](https://github.com/DavidCosbyUofU/slc_examples/tree/main/input_devices/calibration) - Paints the LEDs at each vertex yellow, and and sets the LEDs directly forward, left, right, and backwards white, red, green, and blue, respectively. Useful for calibrating your room the first time, when using real LEDs.
* [Sweep](https://github.com/DavidCosbyUofU/slc_examples/tree/main/input_devices/sweep) - A simple visual effect that sweeps a green light around the room, kinda like a radar.
* [Rainbow]([crates/slc_lab_rainbow](https://github.com/DavidCosbyUofU/slc_examples/tree/main/input_devices/lab_rainbow)) - Uses the mapping feature to map LED direction to their associated color on the CIELAB color wheel.
### Output
* [TCPClient](https://github.com/DavidCosbyUofU/slc_examples/tree/main/output_devices/tcp_client) - Streams LEDs over a TCP connection, frame by frame. See the repository for more information.
* [Gui](https://github.com/DavidCosbyUofU/slc_examples/tree/main/output_devices/slc_gui) - 3D Gui powered by Bevy that lets you preview your room and visual effects. Very convenient if you don't have your LEDs set up yet.
