use bevy::{prelude::*, utils::HashMap, ecs::component, reflect::TypeRegistration};
use serde::{Serialize, Deserialize};

pub struct  RoomLoadingPlugin;

#[derive(Component)]
pub struct RoomTag;

struct Room {
    entities: HashMap<Entity, RoomEntity>,
    //todo add resources and assets
}

#[derive(Debug, Serialize, Deserialize)]
struct RoomEntity {
    name: String,
    components: HashMap<String, String> 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct  RoomComponent {
    data: HashMap<String, String>,
}

impl Plugin for RoomLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_room_system);
    }
}

fn save_room_system(
    input : Res<Input<KeyCode>>,
    enteties : Query<Entity, With<RoomTag>>,
    world: &World,
    type_registry: Res<AppTypeRegistry>,
) {
    if !(input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::S)) {
        return;
    }
    info!("Saving");

    let registry = type_registry.read();

    for entity in enteties.iter() {
        if let Some(entity_ref) = world.get_entity(entity) {
            let archetype = entity_ref.archetype();
            for component_id in archetype.components() {
                
            }
        }
    }
}