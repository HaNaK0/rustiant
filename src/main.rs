use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, save)
        .register_type::<Savable>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    trace!("Setup");
    commands.spawn(Camera2dBundle::default());
    commands.spawn( (SpriteBundle {
        texture: asset_server.load("bevy-icon.png"),
        ..default()
        },
        Savable
    ));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
struct Savable;

fn save(
    entities: Query<Entity, With<Savable>>, 
    world: &World, 
    type_registry: Res<AppTypeRegistry>, 
    keyboard: Res<Input<KeyCode>>) 
{
    if !keyboard.just_pressed(KeyCode::S) {
        return;
    }
    trace!("Saving");
    let mut scene_builder = DynamicSceneBuilder::from_world(world);
    scene_builder.deny::<ComputedVisibility>();
    scene_builder.extract_entities(entities.iter());

    let scene = scene_builder.build();
    let serialized_scene = scene.serialize_ron(&type_registry).unwrap();
    
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/scenes/scene.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

fn load() {
    
}


