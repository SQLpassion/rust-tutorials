mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod random_move;
mod end_turn;

use crate::prelude::*;

// Creates the scheduler for the AwaitingInput state
pub fn build_input_scheduler() -> Schedule
{
    // Add the various systems to the scheduler
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .build()
}

// Creates the scheduler for the PlayerTurn state
pub fn build_player_scheduler() -> Schedule
{
    // Add the various systems to the scheduler
    Schedule::builder()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

// Creates the scheduler for the MonsterTurn state
pub fn build_monster_scheduler() -> Schedule
{
    // Add the various systems to the scheduler
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}