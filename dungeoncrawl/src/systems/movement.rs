use crate::prelude::*;

// Derives a query from the system parameters "entity" and "want_move".
// It runs the system for every matching entity.
#[system(for_each)]       
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,                // The entity from the generated query
    want_move: &WantsToMove,        // The "WantsToMove" message from the generated query
    #[resource]map: &mut Map,       // Returns a reference to the stored Map resource
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

        if let Ok(entry) = ecs.entry_ref(want_move.entity)
        {
            // Get the FieldOfView component of the current entity
            if let Ok(fov) = entry.get_component::<FieldOfView>()
            {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok()
                {
                    camera.on_player_move(want_move.destination);

                    // For all visible tiles, we set the entry in revealed_tiles to true
                    fov.visible_tiles.iter().for_each(|pos|
                    {
                        map.revaled_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }

    // Remove the processed "WantsToMove" message
    commands.remove(*entity);
}