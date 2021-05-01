use slc_core::{
    room::Room,
    room_controller::RoomController,
    util::{LineSegmentTrait, Point},
};
use slc_output::OutputDevice;

use bevy::{pbr::AmbientLight, prelude::*};
use bevy_flycam::{MovementSettings, PlayerPlugin};

const WORLD_SCALE: f32 = 5.0;

//use super::LEDDisplay;
struct SLCSetup;
impl Plugin for SLCSetup {
    fn build(&self, app: &mut AppBuilder) {
        let room = Room::new_from_file("room_configs/config1.rcfg");
        let controller = RoomController { room };

        app.insert_resource(controller);
    }
}

fn lerp(a: Point, b: Point, t: f32) -> Point {
    return (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
}

struct LedID(usize);

fn build_3d(
    mut commands: Commands,
    controller: Res<RoomController>,
    mut ambient_light: ResMut<AmbientLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1 * WORLD_SCALE,
            subdivisions: 1,
        })),
        transform: Transform::from_xyz(
            controller.room.view_pos.0 * WORLD_SCALE,
            2.5 * WORLD_SCALE,
            controller.room.view_pos.1 * WORLD_SCALE,
        ),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..Default::default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.04 * WORLD_SCALE,
            subdivisions: 1,
        })),
        transform: Transform::from_xyz(
            controller.room.view_pos.0 * WORLD_SCALE,
            2.5 * WORLD_SCALE,
            (controller.room.view_pos.1 + 0.1) * WORLD_SCALE,
        ),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..Default::default()
    });

    ambient_light.brightness = 1.0;

    for strip in &controller.room.strips {
        let poleMesh = Mesh::from(shape::Box {
            min_x: -0.05 * WORLD_SCALE,
            max_x: 0.05 * WORLD_SCALE,
            min_y: 0.0 * WORLD_SCALE,
            max_y: 2.5 * WORLD_SCALE,
            min_z: -0.05 * WORLD_SCALE,
            max_z: 0.05 * WORLD_SCALE,
        });

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(poleMesh.clone()),
            transform: Transform::from_xyz(strip.0 .0 * WORLD_SCALE, 0.0, strip.0 .1 * WORLD_SCALE),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        });

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(poleMesh),
            transform: Transform::from_xyz(strip.1 .0 * WORLD_SCALE, 0.0, strip.1 .1 * WORLD_SCALE),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        });
    }

    let mut led_counter = 0;
    let ceiling = &controller.room.num_leds();

    for led in &controller.room.leds {
        let t = led_counter as f32 / *ceiling as f32;
        let led_pos = controller.room.get_pos_at_t(t);

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.01 * WORLD_SCALE,
                    subdivisions: 0,
                })),
                transform: Transform::from_xyz(
                    led_pos.0 * WORLD_SCALE,
                    2.5 * WORLD_SCALE,
                    led_pos.1 * WORLD_SCALE,
                ),
                material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                ..Default::default()
            })
            .insert(LedID(led_counter));

        led_counter += 1;
    }
}

fn direction_test(mut controller: ResMut<RoomController>) {
    controller.set_led_at_dir((0.0, 1.0), (255, 0, 0));
}

fn display_room(
    controller: Res<RoomController>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Handle<StandardMaterial>, &LedID)>,
) {
    for (mat_handle, id) in query.iter() {
        //println!("entered {}", id.0);

        let (r8, g8, b8) = controller.room.leds.get(id.0).unwrap();
        //println!("{}, {}, {}", r8, g8, b8);
        let (r32, g32, b32) = (*r8 as f32 / 255.0, *g8 as f32 / 255.0, *b8 as f32 / 255.0);

        let mat = materials.get_mut(mat_handle).unwrap();
        mat.base_color
            .set(Box::new(Color::rgb(r32, g32, b32)))
            .unwrap();
    }
}

pub struct Gui;

impl OutputDevice for Gui {
    fn start() {
        App::build()
            .add_plugins(DefaultPlugins)
            .add_plugin(SLCSetup)
            //world
            .add_startup_system(build_3d.system())
            // camera
            .add_plugin(PlayerPlugin)
            .insert_resource(MovementSettings {
                sensitivity: 0.0001, // default: 0.00012
                speed: 15.0,         // default: 12.0
            })
            // leds
            .add_startup_system(direction_test.system())
            .add_system(display_room.system())
            .run();
    }
}
