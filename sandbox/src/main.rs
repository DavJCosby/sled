use sled::{color::Rgb, Sled, SledError};
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

fn step(sled: &mut Sled, elapsed: f32) -> Result<(), SledError> {
    sled.map(|led| led.color * 0.9);

    let t = (elapsed / 35.0).cos() / 2.0 + 0.5;

    let p0 = Vec2::new(-2.0, 2.0);
    let p1 = Vec2::new(3.5, 0.0);

    let pos = p0.lerp(p1, t);

    let angle = elapsed / 180.0 % std::f32::consts::TAU;

    sled.set_at_angle_from(angle, pos, Rgb::new(0.0, 1.0, 0.0))?;
    sled.set_at_angle_from(angle + std::f32::consts::PI, pos, Rgb::new(0.0, 1.0, 0.0))?;

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
