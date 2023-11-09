use crate::prelude::*;

// Implements the player input system
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld, 
    #[resource] map: &Map,                      // Returns a reference to the stored Map resource
    #[resource] key: &Option<VirtualKeyCode>,   // Returns a reference to the stored VirtualKeyCode resource
    #[resource] camera: &mut Camera,            // Returns a mutable reference to the stored Camera resource
    #[resource] turn_state: &mut TurnState)     // Returns a mutable reference to the stored TurnState
{
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

        if delta.x != 0 || delta.y != 0
        {
            // Retrieve all the entities with a Point component, and that have a Player tag/component attached.
            // The Player tag/component is not returned, because it is only used as a filter.
            let mut players = <&mut Point>::query().filter(component::<Player>());
        
            // Iterate over each returned Point component
            players.iter_mut(ecs).for_each(|pos|
            {
                let destination = *pos + delta;

                // Check if the tile can be entered by the player
                if map.can_enter_tile(destination)
                {
                    // Change the position of the player accordingly, and reposition the camera
                    *pos = destination;
                    camera.on_player_move(destination);
                    *turn_state = TurnState::PlayerTurn;
                }
            });
        }
    }
}