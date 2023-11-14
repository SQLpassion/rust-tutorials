use crate::prelude::*;

// Implements the end turn system
#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState)
{
    // Return the Health component of the player
    let mut player_hp = <&Health>::query().filter(component::<Player>());
    let current_state = turn_state.clone();

    // Calculate the new turn state
    let mut new_state = match current_state
    {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    // Set the game state to GameOver when the hit points of the player are below 1
    player_hp.iter(ecs).for_each(|hp|
    {
        if hp.current < 1
        {
            new_state = TurnState::GameOver;
        }
    });

    // Store the new turn state
    *turn_state = new_state;
}