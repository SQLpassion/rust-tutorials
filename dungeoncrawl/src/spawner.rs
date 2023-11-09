use crate::prelude::*;

// Adds a new player to the world
pub fn spawn_player(ecs: &mut World, pos: Point)
{
    // Push a new entity to the world
    ecs.push
    (
        // The new entity consists of multiple components
        (
            Player, // Player tag
            pos,    // Point component
            Render  // Render component
            {
                color: ColorPair::new(WHITE, BLACK), 
                glyph: to_cp437('@')
            }
        )
    );
}