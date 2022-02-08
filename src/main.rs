use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

/// Walks around and mutters for himself
#[derive(Component)]
struct Grubbler {
    pub angle: f32,
    pub angle_speed: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform{
                scale: Vec3::new(10.0, 10.0, 10.0),
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Grubbler{angle: 0.0, angle_speed: 1.0});
}

fn grubbler_system(time: Res<Time>, mut query: Query<(&mut Grubbler, &mut Transform)>) {
    for (mut grubbler, mut transform) in query.iter_mut() {
        grubbler.angle += grubbler.angle_speed * time.delta_seconds();
        transform.translation.x = grubbler.angle.cos() * 30.0;
        transform.translation.y = grubbler.angle.sin() * 30.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(grubbler_system)
        .run();
}
