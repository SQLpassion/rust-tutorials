mod player_input;
mod map_render;
mod entity_render;
mod collisions;

use crate::prelude::*;

// Creates the scheduler
pub fn build_scheduler() -> Schedule
{
    // Add the various systems to the scheduler
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}