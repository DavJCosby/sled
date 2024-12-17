[![Crates.io](https://img.shields.io/crates/v/spatial_led.svg)](https://crates.io/crates/spatial_led)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/DavJCosby/sled#License)
[![Docs](https://docs.rs/spatial_led/badge.svg)](https://docs.rs/spatial_led/latest/spatial_led/)
[![Build and Run Tests](https://github.com/DavJCosby/sled/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/DavJCosby/sled/actions/workflows/rust.yml)

# Spatial LED (Sled)
<div> <img src="https://github.com/DavJCosby/sled/blob/master/resources/ripples-demo.gif?raw=true" width="49%" title="cargo run --example ripples"> <img src="https://github.com/DavJCosby/sled/blob/master/resources/warpspeed-demo.gif?raw=true" width="49%" title="cargo run --example warpspeed">
 </div>
Sled is an ergonomic rust library that maps out the shape of your LED strips in 2D space to help you create stunning lighting effects.
<details>
<summary><h5>What Sled does:</h5></summary>

- It exposes an API that lets you:
	- Compute colors depending on each LED's position, distance, direction, line segment, etc;
	- Output colors via a simple, contiguous iterator for your own usage;
	- Filter LEDs by spatial properties to predefine important sets and regions for faster computation;
- Additionally, some tools are provided to help you build functional apps faster (you may opt-out via [compiler features](https://doc.rust-lang.org/cargo/reference/features.html)):
	- [Driver](#drivers) - Pack draw/compute logic into a Driver to simplify the process of swapping between effects, or changing effect settings at runtime. 
	- [Scheduler](#scheduler) - Lightweight tool to schedule redraws at a fixed rate, powered by [spin_sleep](https://github.com/alexheretic/spin-sleep).
</details>

<details>

<summary><h5>What Sled does <ins>not<ins> do:</h5></summary>

- It does not interface directly with your GPIO pins to control your LED hardware. Each project will be different, so it's up to you to bring your own glue. Check out the [Raspberry Pi example](https://github.com/DavJCosby/spatial_led_examples/tree/main/raspberry_pi) to get an idea what that might look like.
- It does not allow you to represent your LEDs in 3D space. Could be a fun idea in the future, but it's just not planned for the time being.

</details>

See the [spatial_led_examples](https://github.com/DavJCosby/spatial_led_examples) repository for examples of Sled in action!

<details open>
<summary><h1>The Basics</h1></summary>

In absence of an official guide, this will serve as a basic introduction. From here, you can consult the [docs](https://docs.rs/spatial_led/latest/spatial_led/) to learn what else Sled can do.
<details open>
<summary><h3>Setup</h3></summary>

To create a Sled struct, you need to create a configuration file and provide its path to the constructor:
```rust
use spatial_led::<Sled, SledError>;
use palette::rgb::Rgb;
fn main() -> Result<(), SledError> {
    let mut sled = Sled::<Rgb>::new("/path/to/config.yap")?;
    Ok(())
}
```

A configuration file explains the layout of your LED strips in 2D space. This is used to pre-calculate some important information, speeding up complex draw calls.

 Example config file:
 ```
center: (0.0, 0.5)
density: 30.0
--segments--
(-2, 0) --> (0.5, -1) --> (3.5, 0) -->
(2, 2) --> (-2, 2) --> (-2, 0)
 ```
 > For more information on how to write config files in this format, check out the [docs](https://docs.rs/spatial_led/latest/spatial_led/struct.Sled.html#method.new).

Note the `::<Rgb>` in the constructor. In previous versions of Sled, [palette's Rgb struct](https://docs.rs/palette/latest/palette/rgb/struct.Rgb.html) was used interally for all color computation. Now, the choice is 100% yours! You just have to specify what data type you'd like to use.

```rust
#[derive(Debug)]
struct RGBW {
    r: f32,
    g: f32,
    b: f32,
    w: f32
}
let mut u8_sled = Sled::<(u8, u8, u8)>::new("/path/to/config.yap")?;
let mut rgbw_sled = Sled::<RGBW>::new("/path/to/config.yap")?;

u8_sled.set(4, (255, 0, 0))?; // set 5th led to red
rgbw_sled.set_all(RGBW {
    r: 0.0,
    g: 1.0,
    b: 0.0,
    w: 0.0
});
```

For all further examples we'll use palette's Rgb struct as our backing color format (we really do highly recommend it and encourage its use wherever it makes sense), but just know that you can use any data type that implements `Debug`, `Default`, and `Copy`.

</details>

<details open>
<summary><h3>Drawing</h3></summary>

Once you have your Sled struct, you can start drawing to it right away! Here’s a taste of some of the things Sled lets you do:

**Set all vertices to white:**
```rust
sled.set_vertices(Rgb::new(1.0, 1.0, 1.0));
```
![Set all Vertices](resources/vertices.png)
> Note that this is a custom terminal UI visualization that is not packaged as part of the sled library. It is ultimately up to you to decide how to visualize your LEDs; Sled just handles the computation.

**Set all LEDs 2 units away from the `center_point` to red:**
```rust
sled.set_at_dist(2.0, Rgb::new(1.0, 0.0, 0.0));
// or relative to any other point using:
// sled.set_at_dist_from(distance, pos, color)
```

![Set at Distance](resources/at_distance.png)

**Set each LED using a function of its direction from point `(2, 1)`:**
```rust
sled.map_by_dir_from(Vec2::new(2.0, 1.0), |dir| {
    let red = (dir.x + 1.0) * 0.5;
    let green = (dir.y + 1.0) * 0.5;
    Rgb::new(red, green, 0.5)
});
```
![Map by Direction](resources/dir_map.png)

**Dim one of the walls by 75%:**
```rust
sled.modulate_segment(3, |led| led.color * 0.25)?;
```
![Modulate Segment](resources/segment_modulate.png)

**Set all LEDs within the overlapping areas of two different circles to blue:**
```rust
let circle_1: Filter = sled.within_dist_from(
    2.0,
    Vec2::new(1.0, 0.5)
);
    
let circle_2: Filter = sled.within_dist_from(
	2.5,
	Vec2::new(-1.0, 1.5)
);

let overlap = circle_1.and(&circle_2);
sled.set_filter(&overlap, Rgb::new(0.0, 0.0, 1.0));
```
![Set Overlapping Areas](resources/filter_and.png)
For more examples, see the documentation comments on the Sled struct.

## Output
Once you’re ready to display these colors, you’ll probably want them packed in a nice contiguous array of RGB values. There are a few methods available to pack the information you need.

```rust
let colors_f32 = sled.colors();
// An Iterator of Rgbs, 32-bits/channel

for color in colors_f32 {
    let red: f32 = color.red;
    // -snip- //
}
```

A few other handy output methods:
```rust
let leds = sled.leds();
// An Iterator of Led structs (holds color, position, distance/angle relative from center, etc)

let positions = sled.positions();
// An Iterator of whatever type you chose to represent colors in.

let positions = sled.positions();
// An Iterator of Vec2s, representing the position of each LED

let colors_and_positions = sled.colors_and_positions();
// An Iterator of (COLOR_TYPE, Vec2) tuple pairs representing each LEDs color and position.
```

</details>
</details>

<details>
<summary><h1>Advanced Features</h1></summary>
For basic applications, the Sled struct gives you plenty of power. Odds are though, you'll want to create more advanced effects that might be time or user-input driven. A few optional (enabled by default, opt-out by disabling their compiler features) tools are provided to streamline that process.

## Drivers
Drivers are useful for encapsulating everything you need to drive a lighting effect all in one place. Here's an example of what a simple one might look like:

```rust
let mut driver = Driver::<Rgb>::new(); // often auto-inferred

driver.set_startup_commands(|_sled, data| {
    data.set::<Vec<Rgb>>("colors", vec![
        Rgb::new(1.0, 0.0, 0.0),
        Rgb::new(0.0, 1.0, 0.0),
        Rgb::new(0.0, 0.0, 1.0),
    ]);
    Ok(())
});

driver.set_draw_commands(|sled, data, time| {
    let elapsed = time.elapsed.as_secs_f32();
    let colors = data.get::<Vec<Rgb>>("colors")?;
    let num_colors = colors.len();
    // clear our canvas each frame
    sled.set_all(Rgb::new(0.0, 0.0, 0.0));

    for i in 0..num_colors {
        let alpha = i as f32 / num_colors as f32;
        let angle = elapsed + (2.0 * PI * alpha);
        sled.set_at_angle(angle, colors[i]);
    }
    Ok(())
});
```
To start using the Driver, give it ownership over a Sled using `.mount()` and use `.step()` to manually refresh it.
```rust
let sled = Sled::<Rgb>::new("path/to/config.yap")?;
driver.mount(sled); // sled gets moved into driver here.

loop {
    driver.step();
    let colors = driver.colors();
    // display those colors however you want
}
```
![Basic Time-Driven Effect](resources/driver1.gif)


`.set_startup_commands()` - Define a function or closure to run when `driver.mount()` is called. Grants mutable control over Sled and Data.

`set_draw_commands()` - Define a function or closure to run every time `driver.step()` is called. Grants mutable control over Sled, and immutable access to Data and Time.

`set_compute_commands()` - Define a function or closure to run every time `driver.step()` is called, scheduled right before draw commands. Grants immutable access to Sled and Time, and mutable control over Data.

If you need to retrieve ownership of your sled later, you can do:
```rust
let sled = driver.dismount();
```

> If you don't need Drivers for your project, you can shed a dependency or two by disabling the `drivers` compiler feature.

For more examples of ways to use drivers, see the [driver_examples folder](https://github.com/DavJCosby/spatial_led_examples/tree/main/driver_examples) in the spatial_led_examples repository.

### Driver Data
A driver exposes a data structure called `Data`. This struct essentially acts as a HashMap of `&str` keys to values of any type you choose to instantiate. This is particularly useful for passing important data and settings in to the effect.

It's best practice to first use startup commands to initialize your data, and then modify them either through compute commands or from outside the driver depending on your needs.

```rust
fn startup(sled: &mut Sled, data: &mut Data) -> SledResult {
    data.set::<Vec<bool>>("wall_toggles", vec![true, false, true]);
    data.set::<Rgb>("room_color", Rgb::new(1.0, 0.0, 0.0));
    data.set("important_data", CustomDataType::new());
    // the compiler can usually infer the data type
    Ok(())
}

driver.set_startup_commands(startup);
```

To access driver data externally, just do:
```rust
let data: &Data = driver.data();
// or
let data: &mut Data = driver.data_mut();
```

Using that data is relatively straightforward.
```rust
let draw_commands = |sled, data, _time| {
    let wall_toggles = data.get::<Vec<bool>>("wall_toggles")?;
    let color = data.get::<Rgb>("room_color")?;
    let important_data: &CustomDataType = data.get("important_data")?;

    for i in 0..wall_toggles.len() {
        if wall_toggles[i] == true {
            sled.set_segment(i, *color)?;
        } else {
            sled.set_segment(i, Rgb::new(0.0, 0.0, 0.0))?;
        }
    }
    
    Ok(())
}
```

If you need to mutate data:
```rust
// Mutable reference to the whole vector
let walls_mut = data.get_mut::<Vec<bool>>("wall_toggles")?;
walls_mut[1] = true;
```

### Filters
For exceptionally performance-sensitive scenarios, Filters can be used to predefine important LED regions. They act as sets, containing only the indicies of the LEDs captured in the set. When we want to perform an operation on that set, we pass the Filter back to the Sled like this:

```rust
let all_due_north: Filter = sled.at_dir(Vec2::new(0.0, 1.0));
sled.for_each_in_filter(&all_due_north, |led| {
    led.color = Rgb::new(1.0, 1.0, 1.0);
});
```
> Note that other methods exist like `.set_filter(filter, color)`, `.modulate_filter(filter, color_rule)`, and `.map_filter(filter, map)`

Once you've stored a Filter, you can save it to `Data` for use in draw/compute stages. Using this pattern, we can pre-compute important sets at startup and then store them to the driver for later usage.

A slightly better example would be to imagine that we have an incredibly expensive mapping function that will only have a visible impact on the LEDs within some radius $R$ from a given point $P$. Rather than checking the distance of each LED from that point every frame, we can instead do something like this:

```rust
let startup_commands = |sled, data| {
    let area: Filter = sled.within_dist_from(
        5.0, Vec2::new(-0.25, 1.5)
    );

    data.set("area_of_effect", area);
    Ok(())
};

let draw_commands = |sled, data, _| {
    let area_filter = data.get("area_of_effect")?;
    sled.map_filter(area_filter, |led| {
        // expensive computation
    });
    Ok(())
};
```
Most `.get` methods on sled will return a Filter, but if you need more precise control you can do something like this:
```rust
let even_filter = sled.filter(|led| led.index() % 2 == 0);
```

Lastly, Filters support a few basic boolean operations, which can be used to combine multiple filters in interesting ways. 

```rust
let circle_a = sled.within_dist_from(
    5.0, Vec2::new(-0.25, 1.5)
);

let circle_b = sled.within_dist_from(
    4.0, Vec2::new(0.5, 0.5)
);

let ab_overlap = circle_a.and(&circle_b);
let ab_union = circle_a.or(&circle_b);
```

## Scheduler
The Scheduler struct makes it super easy to schedule redraws at a fixed rate.

```rust
let mut scheduler = Scheduler::new(120.0);

scheduler.loop_forever(|| {
    driver.step();
});
```
Scheduler, by default, utilizes [spin_sleep](https://crates.io/crates/spin_sleep/) to minimize the high CPU usage you typically see when you spin to wait for the next update by default.

Here are a few other methods that you might also consider:

```rust
// loops until false is returned
scheduler.loop_while_true(|| {
    // -snip- //
    return true;
});

// loops until an error of any type is returned
scheduler.loop_until_err(|| {
    // -snip- //
    Ok(())
});

// best for times when you don't want to pass everything through a closure
loop {
    // -snip- //
    scheduler.sleep_until_next_frame();
}
```
You can define your own `CustomScheduler` backed by whatever sleeping method you prefer if you wish. If you'd like to trim away the `spin_sleep` dependency, you can also disable the `spin_sleep` feature flag.

If you don't need the Scheduler struct in general, you can disable the `scheduler` and `spin_sleep` flags.

</details>
<details>
</summary><h1><code>no_std</code> Support</h1></summary>

Spatial LED is now usable in `no_std` environments as of 0.2.0 (though `alloc` is still required), thanks to some [awesome contributions](https://github.com/DavJCosby/sled/pull/86) by [Claudio Mattera](https://github.com/claudiomattera).

To do this, disable the `std` flag and enable the `libm` flag (for use by glam and palette).

Users on the nightly toolchain can also enable the `core-simd` feature flag for some extra performance if you know your target platform supports SIMD instructions.

## Drivers
The default Driver implementation depends on `std::time::Instant` to track elapsed time between driver steps. For `no_std` environments, you must provide your own struct that implements the `crate::time::Instant` trait.

Once you have that, building a `CustomDriver` becomes as easy as:
```rust
use spatial_led::driver::CustomDriver;

let mut driver = CustomDriver<MyCustomInstant>::new();
driver.mount(sled);
```

## Schedulers
Similarly, the default Scheduler relies on Instants, as well as methods only available through the standard library to handle sleeping. Thus, to build a Scheduler in `no_std` environments, you'll need to provide custom implementations of the `spatial_led::time::Instant` and `spatial_led::time::Sleeper` traits.

```rust
use spatial_led::driver::CustomDriver;

let scheduler = CustomScheduler<MyCustomInstant, MyCustomSleeper>::new(120.0);

 scheduler.loop_forever(|| {
     println!("tick!");
 });
 ```

 As embassy is gaining popularity in the embedded Rust scene, Claudio has also provided an async interface via the `AsyncCustomScheduler` struct.

 ## Feedback and Contributions
 The author of this crate does not own any hardware that would allow him test `spatial_led` on real `no_std` environments, so bug reports and PRs are very appreciated!

</details>

<details>
<summary><h1>Feature Flags</h1></summary>

Enabled by Default:
- `std`
- `drivers` : Enables Drivers
- `scheduler` : Enables Schedulers
- `spin_sleep` : If `std` is enabled, sets the default Scheduler to use [spin_sleep](https://crates.io/crates/spin_sleep) to schedule tasks.

Opt-in:
- `named_colors` : Exposes color constants (for example `spatial_led::color::consts::WHITE`)
- `libm` : Needed for some `no_std` environments.
- `core-simd` (Nightly) : Allows the vector math library used by the crate to take advantage of SIMD instructions when `std::simd` isn't available.
</details>

<details>
<summary><h1>License</h1></summary>

Licensed under either of
* Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
  
at your option.
</details>