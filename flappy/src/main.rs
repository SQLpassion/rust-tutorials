use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

// The current state of the game state machine
enum GameMode
{
    Menu,
    Playing,
    End
}

// The Player structure
struct Player
{
    x: i32,
    y: i32,
    velocity: f32
}

// The implementation of the Player structure
impl Player
{
    // The constructor
    fn new(x: i32, y: i32) -> Self
    {
        Player
        {
            x,
            y,
            velocity: 0.0
        }
    }

    // Renders the player
    fn render(&mut self, ctx: &mut BTerm)
    {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    // Applies gravity to the player
    fn gravity_and_move(&mut self)
    {
        if self.velocity < 2.0
        {
            self.velocity += 0.2
        }

        self.y += self.velocity as i32;
        self.x += 1;

        if self.y < 0
        {
            self.y = 0;
        }
    }

    // Flaps the wings of the player
    fn flap(&mut self)
    {
        self.velocity = -2.0
    }
}

// The Obstacle structure
struct Obstacle
{
    x: i32,         // The x-position of the Obstacle
    gap_y: i32,     // The y-position of the gap of the Obstacle
    size: i32       // The size of the gap
}

// The implementation of the Obstacle structure
impl Obstacle
{
    // The constructor
    fn new (x: i32, score: i32) -> Self
    {
        let mut random = RandomNumberGenerator::new();

        Obstacle
        {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    // Renders the Obstacle
    fn render(&mut self, ctx: &mut BTerm, player_x: i32)
    {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        for y in 0..self.gap_y - half_size
        {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }

        for y in self.gap_y + half_size..SCREEN_HEIGHT
        {
            ctx.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    // Checks if the player has hit an Obstacle
    fn hit_obstacle(&self, player: &Player) -> bool
    {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;

        does_x_match && (player_above_gap || player_below_gap)
    }
}

// The GameState structure
struct State
{
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32
}

// Implement the GameState trait
impl GameState for State
{
    fn tick(&mut self, ctx: &mut BTerm)
    {
        match self.mode
        {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.game_over(ctx),
            GameMode::Playing => self.play(ctx)
        }
    }
}

// The implementation of the GameState structure
impl State
{
    // The constructur
    fn new() -> Self
    {
        State
        {
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle : Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::Menu,
            score: 0
        }
    }

    // The play function
    fn play(&mut self, ctx: &mut BTerm)
    {
        // Set the background
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;

        // Check if we have exceeded the frame duration
        if self.frame_time > FRAME_DURATION
        {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        // Check if the player wants to flap
        if let Some(VirtualKeyCode::Space) = ctx.key
        {
            self.player.flap();
        }

        // Render the player
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        // Render the Obstacle
        self.obstacle.render(ctx, self.player.x);

        // Check if the player has passed the Obstacle
        if self.player.x > self.obstacle.x
        {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }

        // Check for the Game Over condition
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player)
        {
            self.mode = GameMode::End;
        }
    }

    // Restarts the game
    fn restart(&mut self)
    {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }

    // Shows the main menu
    fn main_menu(&mut self, ctx: &mut BTerm)
    {
        // Displays the main menu
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key
        {
            match key
            {
                // Play the game
                VirtualKeyCode::P => self.restart(),

                // Quit the game
                VirtualKeyCode::Q => ctx.quitting = true,

                // Anything else
                _ => {}
            }
        }
    }

    // Displays the Game Over menu
    fn game_over(&mut self, ctx: &mut BTerm)
    {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key
        {
            match key
            {
                // Play the game
                VirtualKeyCode::P => self.restart(),

                // Quit the game
                VirtualKeyCode::Q => ctx.quitting = true,

                // Anything else
                _ => {}
            }
        }
    }
}

fn main() -> BError
{
    // Create a new Bracket-Lib context
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .with_fitscreen(true)
        .build()?;

    // Star the game loop with the associated GameState
    main_loop(context, State::new())
}