// Creates the various modules
mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

// Defines the prelude
mod prelude
{
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

// Use everything from the prelude
use prelude::*;

// Stores the Game state
struct State
{
    ecs: World,
    resources: Resources,
    systems: Schedule
}

// Implementation of the Game state structure
impl State
{
    // Constructor
    fn new() -> Self
    {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Adds a new player to the world
        spawn_player(&mut ecs, map_builder.player_start);

        // In each room we also add a new monster to the world
        map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

        // Inserts the map and the camera as resources
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        
        Self
        {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

// Implements the GameState trait
impl GameState for State
{
    fn tick(&mut self, ctx: &mut BTerm)
    {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();

        // Add the current keyboard state as a new resource
        // It replaces the old keyboard state resource, because we can only store one instance of a specific resource type
        self.resources.insert(ctx.key);

        // Execute the various registered systems
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render everything
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError
{
    // Create a new game window
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    // Runs the main loop of the game
    main_loop(context, State::new())
}