use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType
{
    Wall,
    Floor
}

// The game map
pub struct Map
{
    // A list of tiles
    pub tiles: Vec<TileType>
}

// The implementation of the game map
impl Map
{
    // The constructor.
    pub fn new() -> Self
    {
        Self
        {
            // Creates a new map where all the tiles are Floor types
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    // Checks if a given x/y coordinate is within the map
    pub fn in_bounds(&self, point: Point) -> bool
    {
        point.x >= 0 && point.x < SCREEN_WIDTH &&
            point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Checks if a given tile can be entered by the player on the map
    pub fn can_enter_tile(&self, point: Point) -> bool
    {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    // Checks if a given x/y coordinate is within the map.
    // If yes, the linear index of the coordinate is returned, otherwise None is returned.
    pub fn try_idx(&self, point: Point) -> Option<usize>
    {
        if !self.in_bounds(point)
        {
            None
        }
        else
        {    
            Some(map_idx(point.x, point.y))
        }
    }
}

// Maps a x/y coordinate to a linear index into the TileMap array
pub fn map_idx(x: i32, y: i32) -> usize
{
    ((y * SCREEN_WIDTH) + x) as usize
}