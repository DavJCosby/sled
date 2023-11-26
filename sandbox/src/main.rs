use sled::{
    color::{chromatic_adaptation::AdaptInto, Rgb},
    Sled, SledError,
};

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

const NUM_FAIRIES: usize = 12;

fn step(sled: &mut Sled, elapsed: f32) -> Result<(), SledError> {
    sled.for_each(|led| {
        led.color *= Rgb::new(0.9, 0.93, 0.99);
    });

    let center = sled.center_point();
    let dist = (elapsed / 30.0) % 4.0;
    sled.map(|led| {
        let d = led.position().distance(center);
        let dd = d - dist;
        if (0.0..=0.05).contains(&dd) {
            return Rgb::new(0.0, 1.0, 0.5);
        }
        led.color
    });

    let closest = sled.get_closest_to_mut(center);
    closest.color = Rgb::new(1.0, 1.0, 1.0);

    // for i in 0..NUM_FAIRIES {
    //     let c = sled::color::Oklch::new(1.0, 0.9, elapsed + 20.0 * i as f32).adapt_into();

    //     sled.set_at_angle(
    //         (elapsed + (360.0 / NUM_FAIRIES as f32) * i as f32 % 360.0).to_radians(),
    //         c,
    //     )?;
    // }

    Ok(())
}

fn update_colors(mut sled: ResMut<SledResource>, time: Res<Time>) {
    let elapsed = time.elapsed_seconds_wrapped() * 80.0;
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
                mesh: meshes.add(shape::Circle::new(3.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::rgb(r, g, b))),
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
                ..default()
            },
            LedIndex(led.index()),
        ));
    }

    let center = sled.0.center_point() * 150.0;

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(16.).into()).into(),
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
