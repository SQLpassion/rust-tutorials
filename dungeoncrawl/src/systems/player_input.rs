use crate::prelude::*;

// Implements the player input system
#[system]
#[read_component(Point)]
#[read_component(Player)]
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

        // Iterate over each player (there is just one)
        players.iter(ecs).for_each(|(entity, pos)|
        {
            // Calculate the new position
            let destination = *pos + delta;

            // "Send" a WantsToMove message
            commands.push(((), WantsToMove { entity: *entity, destination}));
        });

        // Set the new turn state
        *turn_state = TurnState::PlayerTurn;
    }
}