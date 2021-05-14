use bevy::{pbr::AmbientLight, prelude::*};
use bevy_flycam::PlayerPlugin;
use slc::devices::OutputDevice;
use slc::room_controller::RoomController;

use std::sync::{Arc, RwLock};

const WORLD_SCALE: f32 = 5.0;
const CEILING_HEIGHT: f32 = 2.7432;
const LED_SIZE: f32 = 0.007;

struct LedID(usize);

struct WorldBuilder;
impl Plugin for WorldBuilder {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(build_base_world.system())
            .add_startup_system(build_view_orb.system())
            .add_startup_system(build_poles.system())
            .add_startup_system(build_leds.system());
    }
}

fn build_view_orb(
    mut commands: Commands,
    locked_controller: Res<Arc<RwLock<RoomController>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let controller_read_only = locked_controller.read().unwrap();

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.1 * WORLD_SCALE,
            subdivisions: 1,
        })),
        transform: Transform::from_xyz(
            controller_read_only.room.view_pos.0 * WORLD_SCALE,
            controller_read_only.room.view_pos.1 * WORLD_SCALE,
            CEILING_HEIGHT * WORLD_SCALE,
        ),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        ..Default::default()
    });

    drop(controller_read_only);
}

fn build_poles(
    mut commands: Commands,
    locked_controller: Res<Arc<RwLock<RoomController>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let controller_read_only = locked_controller.read().unwrap();
    for strip in &controller_read_only.room.strips {
        let pole_mesh = Mesh::from(shape::Box {
            min_x: -0.05 * WORLD_SCALE,
            max_x: 0.05 * WORLD_SCALE,
            min_y: -0.05 * WORLD_SCALE,
            max_y: 0.05 * WORLD_SCALE,
            min_z: 0.0 * WORLD_SCALE,
            max_z: CEILING_HEIGHT * WORLD_SCALE,
        });

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(pole_mesh.clone()),
            transform: Transform::from_xyz(strip.0 .0 * WORLD_SCALE, strip.0 .1 * WORLD_SCALE, 1.0),
            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            ..Default::default()
        });

        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(pole_mesh),
            transform: Transform::from_xyz(strip.1 .0 * WORLD_SCALE, strip.1 .1 * WORLD_SCALE, 0.0),
            material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
            ..Default::default()
        });
    }
    drop(controller_read_only);
}

fn build_leds(
    mut commands: Commands,
    locked_controller: Res<Arc<RwLock<RoomController>>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let controller_read_only = locked_controller.read().unwrap();

    let mut led_counter = 0;
    let ceiling = controller_read_only.room.num_leds();

    for _ in &controller_read_only.room.leds {
        let t = led_counter as f32 / ceiling as f32;
        let led_pos = controller_read_only.room.get_pos_at_t(t);

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: LED_SIZE * WORLD_SCALE,
                    subdivisions: 1,
                })),
                transform: Transform::from_xyz(
                    led_pos.0 * WORLD_SCALE,
                    led_pos.1 * WORLD_SCALE,
                    CEILING_HEIGHT * WORLD_SCALE,
                ),
                material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
                ..Default::default()
            })
            .insert(LedID(led_counter));

        led_counter += 1;
    }

    drop(controller_read_only);
}

fn build_base_world(mut ambient_light: ResMut<AmbientLight>) {
    ambient_light.brightness = 1.0;
}

fn refresh_leds(
    locked_controller: Res<Arc<RwLock<RoomController>>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Handle<StandardMaterial>, &LedID)>,
) {
    let controller_read_only = locked_controller.read().unwrap();
    for (mat_handle, id) in query.iter() {
        let (r8, g8, b8) = controller_read_only.room.leds.get(id.0).unwrap();
        let (r32, g32, b32) = (*r8 as f32 / 255.0, *g8 as f32 / 255.0, *b8 as f32 / 255.0);

        let mat = materials.get_mut(mat_handle).unwrap();
        mat.roughness = 1.0;
        mat.emissive
            .set(Box::new(Color::rgb(r32, g32, b32)))
            .unwrap();
    }

    drop(controller_read_only);
}

pub struct Gui;

impl Gui {
    pub fn new() -> Gui {
        Gui
    }
}

impl OutputDevice for Gui {
    fn start(&self, locked_controller: Arc<RwLock<RoomController>>) {
        App::build()
            .insert_resource(locked_controller)
            .insert_resource(Msaa { samples: 4 })
            .add_plugins(DefaultPlugins)
            .add_plugin(WorldBuilder)
            .add_plugin(PlayerPlugin)
            // leds
            .add_system(refresh_leds.system())
            .run();
    }
}
