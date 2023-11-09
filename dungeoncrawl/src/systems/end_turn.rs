use crate::prelude::*;

// Implements the end turn system
#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState)
{
    // Calculate the new turn state
    let new_state = match turn_state
    {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput
    };

    // Store the new turn state
    *turn_state = new_state;
}