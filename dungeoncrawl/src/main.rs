// Creates the various modules
mod map;
mod map_builder;
mod player;

// Defines the prelude
mod prelude
{
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
}

// Use everything from the prelude
use prelude::*;

// Stores the Game state
struct State
{
    // The map
    map: Map,

    // The player
    player: Player
}

// Implementation of the Game state structure
impl State
{
    // Constructor
    fn new() -> Self
    {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        
        Self
        {
            map: map_builder.map,
            player: Player::new(map_builder.player_start)
        }
    }
}

// Implements the GameState trait
impl GameState for State
{
    fn tick(&mut self, ctx: &mut BTerm)
    {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError
{
    // Create a new game window
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_fitscreen(true)
        .build()?;

    // Runs the main loop of the game
    main_loop(context, State::new())
}