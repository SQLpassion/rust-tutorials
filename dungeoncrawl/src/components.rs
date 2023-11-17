use std::collections::HashSet;
pub use crate::prelude::*;

// The Render component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render
{
    pub color: ColorPair,
    pub glyph: FontCharType
}

// The Player component, which is just a simple tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

// The Enemy component, which is just a simple tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

// The MovingRandomly component, which is just a simple tag
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

// The WantsToMove component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove
{
    pub entity: Entity,
    pub destination: Point
}

// The Health component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health
{
    pub current: i32,
    pub max: i32
}

// The Name component
#[derive(Clone, PartialEq)]
pub struct Name(pub String);

// The WantsToAttack component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack
{
    pub attacker: Entity,
    pub victim: Entity
}

// The ChasingPlayer component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

// The Item component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

// The AmuletOfYala component
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

// The FieldOfView component
#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView
{
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool
}

// The implementation of the FieldOfView component
impl FieldOfView
{
    // The constructor
    pub fn new(radius: i32) -> Self
    {
        Self
        {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true
        }
    }

    // A simple clone function
    pub fn clone_dirty(&self) -> Self
    {
        Self
        {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
        }
    }
}