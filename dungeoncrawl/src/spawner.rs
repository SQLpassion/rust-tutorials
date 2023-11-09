use crate::prelude::*;

// Adds a new player to the world
pub fn spawn_player(ecs: &mut World, pos: Point)
{
    // Push a new entity to the world
    ecs.push
    (
        // The new entity consists of multiple components which are structured in a tuple
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

// Adds a new monster to the world
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point)
{
    // Push a new entity to the world
    ecs.push
    (
        // The new entity consists of multiple components which are structured in a tuple
        (
            Enemy,          // Enemy tag 
            pos,            // Point component
            MovingRandomly, // Random Movement tag
            Render          // Render component
            {
                color: ColorPair::new(WHITE, BLACK),
                glyph: match rng.range(0, 4)
                {
                    0 => to_cp437('E'),
                    1 => to_cp437('O'),
                    2 => to_cp437('o'),
                    _ => to_cp437('g')
                }
            }
        )    
    );
}