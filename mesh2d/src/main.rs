use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

fn main() {
    println!("Hello, Bevy!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(mesh2d)
        .run();

}

fn mesh2d(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(240.)),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..default()
    });
}
