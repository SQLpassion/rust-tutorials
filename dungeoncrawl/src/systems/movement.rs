use crate::prelude::*;

// Derives a query from the system parameters "entity" and "want_move".
// It runs the system for every matching entity.
#[system(for_each)]                 
#[read_component(Player)]
pub fn movement(
    entity: &Entity,                // The entity from the generated query
    want_move: &WantsToMove,        // The "WantsToMove" message from the generated query
    #[resource]map: &Map,           // Returns a reference to the stored Map resource
    #[resource]camera: &mut Camera, // Returns a reference to the stored Camera resource
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer)
{
    // Check if the new destination on the map can be entered
    if  map.can_enter_tile(want_move.destination)
    {
        // Add the entity with the new destination.
        // This will replace the old component.
        commands.add_component(want_move.entity, want_move.destination);

        // Check if the entity is a player that wants to move
        if ecs.entry_ref(want_move.entity).unwrap().get_component::<Player>().is_ok()
        {
            // Recenter the camera on the player
            camera.on_player_move(want_move.destination);
        }
    }

    // Remove the processed "WantsToMove" message
    commands.remove(*entity);
}