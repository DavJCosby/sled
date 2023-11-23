use sled::{color::Rgb, Sled, SledError};

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
struct LedIndex(usize);

fn main() -> Result<(), SledError> {
    let sled = Sled::new("./cfg/config1.toml")?;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SledResource(sled))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_colors, display_colors))
        .run();

    Ok(())
}

fn update_colors(mut sled: ResMut<SledResource>, time: Res<Time>) {
    let elapsed = time.elapsed_seconds_wrapped() * 70.0;
    sled.0.for_each(|led| {
        led.color *= Rgb::new(0.92, 0.975, 0.98);
    });

    sled.0
        .set_at_angle(
            elapsed % 360.0,
            Rgb::new(1.0, 1.0, 1.0),
        )
        .unwrap();
    sled.0
        .set_at_angle(
            (elapsed + 90.0) % 360.0,
            Rgb::new(1.0, 1.0, 1.0),
        )
        .unwrap();

        sled.0
        .set_at_angle(
            (elapsed + 180.0) % 360.0,
            Rgb::new(1.0, 1.0, 1.0),
        )
        .unwrap();

        sled.0
        .set_at_angle(
            (elapsed + 270.0) % 360.0,
            Rgb::new(1.0, 1.0, 1.0),
        )
        .unwrap();
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
