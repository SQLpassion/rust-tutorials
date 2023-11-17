use crate::prelude::*;

// Implements the FieldOfView system
#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map)
{
    // Retrieve all the entities with a Point and FieldOfView component
    let mut views = <(&Point, &mut FieldOfView)>::query();

    views
        .iter_mut(ecs)                      // Returns a mutable iterator
        .filter(|(_, fov)| fov.is_dirty)    // Filter on dirty FieldOfView components
        .for_each(|(pos, fov)|              // Iterate over the entities
        {
            // Calculate for the current FieldOfView component the visible tiles
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });
}