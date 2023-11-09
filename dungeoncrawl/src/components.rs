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