use slc_core::{room::Room, room_controller::RoomController};
use slc_output::OutputDevice;

use bevy::prelude::*;
use bevy_flycam::{MovementSettings, PlayerPlugin};

//use super::LEDDisplay;
struct SLCSetup;
impl Plugin for SLCSetup {
    fn build(&self, app: &mut AppBuilder) {
        let room = Room::new_from_file("room_configs/config1.rcfg");
        let controller = RoomController { room };

        app.insert_resource(controller);
    }
}

fn build_3d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..Default::default()
    });
}

fn direction_test(mut controller: ResMut<RoomController>) {
    controller.set_led_at_dir((1.0, 0.0), (1, 1, 1));
}

fn display_room(controller: Res<RoomController>) {
    let leds = &controller.room.leds;
    for led in leds {
        println!("{:?}", led);
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
            //.add_system(display_room.system())
            .run();
    }
}
