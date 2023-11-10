use crate::prelude::*;

// Implements the Random Movement system
#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer)
{
    // Retrieve all the entities that have a Point and MovingRandomly component
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    // Iterate over each monster
    movers.iter(ecs).for_each(|(entity, pos, _)|
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

        // "Send" a WantsToMove message
        commands.push(((), WantsToMove { entity: *entity, destination}));
    });
}