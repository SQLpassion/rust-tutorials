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

    // Checks if a given destination tile is a valid exit.
    // If we can enter that tile, it's a valid exit, otherwise it's not a valid exit.
    fn valid_exit(
        &self,
        loc: Point,     // The current tile
        delta: Point    // The movement delta
    ) -> Option<usize>
    {
        // Calculate the new destination tile
        let destination = loc + delta;

        // Check if the destination tile is within the map
        if self.in_bounds(destination)
        {
            // Check if we can enter the destination tile
            if self.can_enter_tile(destination)
            {
                // It's a valid exit
                let idx = self.point2d_to_index(destination);
                Some(idx)
            }
            else
            { 
                // It's not a valid exit
                None
            }
        }
        else
        {   
            // It's not a valid exit
            None
        }
    }
}

// Implements the Algorithm2D trait
impl Algorithm2D for Map
{
    // Returns the dimensions of the map
    fn dimensions(&self) -> Point
    {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    // Checks if a point is within the map dimensions
    fn in_bounds(&self, point: Point) -> bool
    {
        self.in_bounds(point)
    }
}

// Implements the BaseMap trait
impl BaseMap for Map
{
    // Get all possible exits for a given tile
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]>
    {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        // Moving west...
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0))
        {
            exits.push((idx, 1.0));
        }

        // Moving east...
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0))
        {
            exits.push((idx, 1.0));
        }

        // Moving south...
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1))
        {
            exits.push((idx, 1.0));
        }

        // Moving north...
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1))
        {
            exits.push((idx, 1.0));
        }

        // Return all possible exits
        exits
    }

    // Calculates the distance between 2 given points
    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32
    {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))    
    }
}

// Maps a x/y coordinate to a linear index into the TileMap array
pub fn map_idx(x: i32, y: i32) -> usize
{
    ((y * SCREEN_WIDTH) + x) as usize
}