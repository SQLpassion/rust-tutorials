use crate::prelude::*;

// Adds a new player to the world
pub fn spawn_player(ecs: &mut World, pos: Point)
{
    // Push a new entity to the world
    ecs.push
    (
        // The new entity consists of multiple components which are structured in a tuple
        (
            Player, // Player tag
            pos,    // Point component
            Render  // Render component
            {
                color: ColorPair::new(WHITE, BLACK), 
                glyph: to_cp437('@')
            },
            Health // Health component
            {
                current: 100,
                max: 100
            },
            FieldOfView::new(8) // FieldOfView component
        )
    );
}

// Adds a new monster to the world
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point)
{
    // Creates a random number between 1 and 10, and returns
    // a tuple with the information about the given monster
    let (hp, name, glyph) = match rng.roll_dice(1, 10)
    {
        1..=8 => goblin(),  // Returns a Goblin
        _ => orc()          // Returns an Orc
    };

    // Push a new entity to the world
    ecs.push
    (
        (
            Enemy,          // Enemy tag 
            ChasingPlayer,  // Chasing Player tag
            pos,            // Point component
            Render          // Render component
            {
                color: ColorPair::new(WHITE, BLACK), glyph
            },
            Health { current: hp, max: hp}, // Health component
            Name(name),                     // Name component
            FieldOfView::new(6)             // FieldOfView component
        )
    );
}

// Adds a new Amulet to the world
pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point)
{
    // Push a new entity to the world
    ecs.push
    (
        (
            Item,           // Item tag
            AmuletOfYala,   // AmuletOfYala tag
            pos,            // Point component
            Render          // Render component
            {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|')
            },
            Name("Amulet of Yala".to_string())
        )
    );
}

// Returns a new Goblin
// It returns a tuble consisting of (HitPoint, Name, CharacterToRender)
fn goblin() -> (i32, String, FontCharType)
{
    (1, "Goblin".to_string(), to_cp437('g'))
}

// Retunrs a new Orc
fn orc() -> (i32, String, FontCharType)
{
    (2, "Orc".to_string(), to_cp437('o'))
}