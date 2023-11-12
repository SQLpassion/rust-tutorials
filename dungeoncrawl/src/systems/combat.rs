use crate ::prelude::*;

// Implements the Combat system
#[system]
#[read_component(WantsToAttack)]
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
        // Get the Health component of the victim
        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;

            if health.current < 1
            {
                commands.remove(*victim);
            }

            println!("Health after attack: {}", health.current);
        }

        commands.remove(*message);
    });
}