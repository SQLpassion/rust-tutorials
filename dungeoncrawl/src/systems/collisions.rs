use crate::prelude::*;

// Implements the collision detection system
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer)
{
    // Retrieve the current position of the player
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    // Retrieve all the enemies
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    enemies.iter(ecs)                           // Iterate over all enemies
        .filter(|(_, pos)| **pos == player_pos) // Filter only on the enemies that match the current player's position
        .for_each                               // Iterate over the remaining enemies
        (
            |(entity, _)|
            {
                commands.remove(*entity);       // Remove the enemy
            }
        );
}