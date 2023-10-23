use crate::prelude::*;

// The player of the game
pub struct Player
{
    // The position of the player
    pub position: Point
}

// The implementation of the player
impl Player
{
    // The constructor
    pub fn new(position: Point) -> Self
    {
        Self
        {
            position
        }
    }

    // Renders the player
    pub fn render(&self, ctx: &mut BTerm)
    {
        ctx.set(self.position.x, self.position.y, WHITE, BLACK, to_cp437('@'));
    }

    // Updates the position of the player
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map)
    {
        if let Some(key) = ctx.key
        {
            // Calculate the delta of the new player position
            let delta = match key
            {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };

            // Calculate the new player position
            let new_position = self.position + delta;

            // Check if the new player position can be entered
            if map.can_enter_tile(new_position)
            {
                self.position = new_position;
            }
        }
    }
}