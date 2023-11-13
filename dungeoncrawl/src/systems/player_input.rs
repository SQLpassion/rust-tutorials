use crate::prelude::*;

// Implements the player input system
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
pub fn player_input(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,   // Returns a reference to the stored VirtualKeyCode resource
    #[resource] turn_state: &mut TurnState)     // Returns a mutable reference to the stored TurnState
{
    // Retrieve all the entities with a Point component, and that have a Player tag/component attached.
    // The Player tag/component is not returned, because it is only used as a filter.
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = key
    {
        let delta = match key
        {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0)
        };

        // Retrieve the player with its destination
        let (player_entity, player_destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        // Retrieve all the enemies
        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        // Stores if the Player did something
        // If false, we grant the Player some additional health points
        let mut did_something = false;

        if delta.x != 0 || delta.y != 0
        {
            let mut hit_something = false;

            enemies
                .iter(ecs)
                .filter(|(_, enemy_pos)|
                {
                    // Check if the position of the enemy matches with the new position of the player
                    **enemy_pos == player_destination
                })
                .for_each(|(enemy, _)|
                {
                    hit_something = true;
                    did_something = true;

                    // Send a WantsToAttack message
                    commands.push(((), WantsToAttack { attacker: player_entity, victim: *enemy }));
                });

            if !hit_something
            {
                did_something = true;

                // Send a WantsToMove message, when we don't hit any enemy during the player movement
                commands.push(((), WantsToMove { entity: player_entity, destination: player_destination }));
            }
        }

        if !did_something
        {
            if let Ok(health) = ecs
                .entry_mut(player_entity)
                .unwrap()
                .get_component_mut::<Health>()
            {
                health.current = i32::min(health.max, health.current + 1);
            }
        }

        // Set the new turn state
        *turn_state = TurnState::PlayerTurn;
    }
}