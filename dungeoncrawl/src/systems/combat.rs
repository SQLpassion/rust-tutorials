use crate ::prelude::*;

// Implements the Combat system
#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(
    ecs: &mut SubWorld, 
    commands: &mut CommandBuffer)
{
    // Returns all the entities that want to attack
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    // Returns a list of the victims
    let victims : Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();

    // Iterate over each victim
    victims.iter().for_each(|(message, victim)|
    {
        // Check if the victim is a player
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        // Get the Health component of the victim
        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;

            if health.current < 1 && !is_player
            {
                commands.remove(*victim);
            }
        }

        commands.remove(*message);
    });
}