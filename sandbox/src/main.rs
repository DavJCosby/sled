use std::f32::consts::TAU;

use sled::{Rgb, Sled, SledError};

use bevy::{
    audio::AudioPlugin, diagnostic::DiagnosticsPlugin, gltf::GltfPlugin, log::LogPlugin,
    prelude::*, scene::ScenePlugin, sprite::MaterialMesh2dBundle, text::TextPlugin, ui::UiPlugin,
};

#[derive(Component)]
struct LedIndex(usize);

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./cfg/config1.toml")?;

    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .disable::<HierarchyPlugin>()
                .disable::<DiagnosticsPlugin>()
                .disable::<GilrsPlugin>()
                .disable::<UiPlugin>()
                .disable::<ScenePlugin>()
                .disable::<TextPlugin>()
                .disable::<UiPlugin>()
                .disable::<GltfPlugin>()
                .disable::<AudioPlugin>()
                .disable::<GilrsPlugin>()
                .disable::<AnimationPlugin>(),
        )
        .insert_resource(SledResource(sled))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_colors, display_colors))
        .run();

    Ok(())
}

const GREEN_RADIUS: f32 = 35.0;
const GREEN_COUNT: usize = 64;

const BLUE_RADIUS: f32 = 45.0;
const BLUE_COUNT: usize = 96;

const TRAIL_RADIUS: f32 = 18.0;

fn step(sled: &mut Sled, elapsed: f32) -> Result<(), SledError> {
    let inner_color = Rgb::new(0.6, 0.93, 0.762);
    let outer_delta = Rgb::new(0.4, 0.51, 0.93);

    let inner_time_scale = elapsed / GREEN_RADIUS;
    let outer_time_scale = elapsed / BLUE_RADIUS;

    for i in 0..GREEN_COUNT {
        let angle = inner_time_scale + (TAU / GREEN_COUNT as f32) * i as f32;
        sled.modulate_at_angle(angle, |led| led.color + inner_color)
            .unwrap();
    }

    for i in 0..BLUE_COUNT {
        let angle = outer_time_scale + (TAU / BLUE_COUNT as f32) * i as f32 % TAU;
        sled.modulate_at_angle(angle, |led| led.color + outer_delta)
            .unwrap();
    }

    let radar_time_scale = elapsed / TRAIL_RADIUS;
    let angle = radar_time_scale % TAU;
    sled.map(|led| {
        let da = (led.angle() + angle) % TAU;
        let fac = 1.0 - (da / (TAU)).powf(1.25);
        led.color * fac
    });

    Ok(())
}

fn update_colors(mut sled: ResMut<SledResource>, time: Res<Time>) {
    let elapsed = time.elapsed_seconds_wrapped() * 20.0;
    let sled = &mut sled.0;

    step(sled, elapsed).unwrap();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sled: Res<SledResource>,
) {
    commands.spawn(Camera2dBundle::default());

    for led in sled.0.read() {
        let (r, g, b) = led.color.into_components();
        let position = led.position() * 150.0;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(2.5).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(r, g, b))),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                ..default()
            },
            LedIndex(led.index()),
        ));
    }

    let center = sled.0.center_point() * 150.0;

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(12.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.25, 0.25, 0.25))),
        transform: Transform::from_translation(Vec3::new(center.x, center.y, 0.)),
        ..default()
    });
    // Circle
}

fn display_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    sled: Res<SledResource>,
    query: Query<(&mut Handle<ColorMaterial>, &LedIndex)>,
) {
    for (material, index) in query.iter() {
        if let Some(material) = materials.get_mut(material) {
            let (r, g, b) = sled.0.get(index.0).unwrap().color.into_components();
            material.color = Color::rgb(r, g, b);
        }
    }
}

#[derive(Resource)]
struct SledResource(Sled);
