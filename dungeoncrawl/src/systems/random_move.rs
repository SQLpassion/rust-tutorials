use crate::prelude::*;

// Implements the Random Movement system
#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map)
{
    // Retrieve all the entities that have a Point and MovingRandomly component
    let mut movers = <(&mut Point, &MovingRandomly)>::query();

    movers
        .iter_mut(ecs)
        .for_each(|(pos, _)|
        {
            let mut rng = RandomNumberGenerator::new();

            // Calculate randomly a new destination
            let destination = match rng.range(0, 4)
            {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1)
            } + *pos;

            // Check if the new destination can be entered
            if map.can_enter_tile(destination)
            {
                // Store the new destination in the world
                *pos = destination;
            }
        }
    );
}