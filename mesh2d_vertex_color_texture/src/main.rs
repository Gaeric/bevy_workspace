use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

fn main() {
    println!("Hello, Mesh2d With Texture!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("banner.png");
    let mut mesh = Mesh::from(shape::Quad::default());

    let vertext_colors: Vec<[f32; 4]> = vec![
        Color::RED.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
        Color::WHITE.as_rgba_f32(),
    ];

    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertext_colors);

    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(mesh).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(ColorMaterial::from(texture_handle)),
        ..default()
    });
}
