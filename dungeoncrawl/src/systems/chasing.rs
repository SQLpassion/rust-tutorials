use crate::prelude::*;

// Implements the Chasing system
#[system]
#[read_component(Point)]
#[read_component(ChasingPlayer)]
#[read_component(Health)]
#[read_component(Player)]
pub fn chasing(
    #[resource] map: &Map,
    ecs: &SubWorld,
    commands: &mut CommandBuffer)
{
    // Return all the monsters that are chasing the player
    let mut monsters = <(Entity, &Point, &ChasingPlayer)>::query();

    // Return all entities with a Point and Health component (player and monsters)
    let mut positions = <(Entity, &Point, &Health)>::query();

    // Return the players position
    let mut player = <(&Point, &Player)>::query();

    // Return the player position
    let player_pos = player.iter(ecs).nth(0).unwrap().0;
    let player_idx = map_idx(player_pos.x, player_pos.y);

    // Creates a new Dijkstra map with the player as a starting position
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &search_targets,
        map,
        1024.0
    );

    // Iterate over each monster
    monsters.iter(ecs).for_each(|(entity, monster_pos, _)|
    {
        let idx = map_idx(monster_pos.x, monster_pos.y);

        // Find for the current monster the tile with the lowest exit value (based on the generated Dijkstra Map)
        // The lowest exit value will guide the monster *towards* the current location of the player
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map)
        {
            // Calculate the distance between the monster and the player
            let distance = DistanceAlg::Pythagoras.distance2d(*monster_pos, *player_pos);

            let destination = if distance > 1.2
            {
                // The distance is larger than one, so the new position of the monster
                // is the tile with the lowest exit value
                map.index_to_point2d(destination)
            }
            else
            {
                // The monster is at the same tile as the player
                *player_pos
            };

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
        }
    });
}