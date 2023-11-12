use crate::prelude::*;

// Implements the Random Movement system
#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
#[read_component(Health)]
#[read_component(Player)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer)
{
    // Retrieve all the entities that have a Point and MovingRandomly component
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    // Retrieve all the entities that have a Point and Health component
    let mut positions = <(Entity, &Point, &Health)>::query();

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

        let mut attacked = false;

        // Iterate over all the entities that have a Point and Health component
        positions
            .iter(ecs)
            .filter(|(_, target_pos, _)| **target_pos == destination) // Check if the new position of the Monster matches with the position of the Player
            .for_each(|(victim, _, _)|
            {
                // Check if the victim is a Player
                if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok()
                {
                    // If yes, the Monster attacks now the Player
                    commands.push(((), WantsToAttack { attacker: *entity, victim: *victim }));
                }

                attacked = true;
            });

        if !attacked
        {
            // Send a WantsToMove message
            commands.push(((), WantsToMove { entity: *entity, destination}));
        }
    });
}