use crate::prelude::*;

// Implements the end turn system
#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(AmuletOfYala)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState)
{
    // Return the Health and position component of the player
    let mut player_hp = <(&Health, &Point)>::query().filter(component::<Player>());

    // Return the position of the amulet
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    // Clone the current state
    let current_state = turn_state.clone();

    // Calculate the new turn state
    let mut new_state = match current_state
    {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    player_hp.iter(ecs).for_each(|(hp, pos)|
    {
        // Set the game state to GameOver when the hit points of the player are below 1
        if hp.current < 1
        {
            new_state = TurnState::GameOver;
        }

        // Set the game state to Victory if the player has found the amulet
        if pos == amulet_pos
        {
            new_state = TurnState::Victory;
        }
    });

    // Store the new turn state
    *turn_state = new_state;
}