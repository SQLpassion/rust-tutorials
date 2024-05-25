use crate::prelude::*;
use super::MapArchitect;

const LEVEL: (&str, i32, i32) = 
(
"
----------------------
---##--##-------------
---#----#-------------
---#-M--#-------------
-###----#############-
--M------M------------
-###----#############-
---#----#-------------
---#----#-------------
---######-------------
----------------------
", 
    22,
    11
);

pub struct LevelArchitect
{
}

// Implement the MapArchitect trait
impl MapArchitect for LevelArchitect
{
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder
    {
        let mut mb = MapBuilder
        {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero()
        };

        // mb.fill(TileType::Floor);
        // mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.player_start = Point::new(1, 1);
        mb.amulet_start = mb.find_most_distant();

        // Convert the level definition into a vector of characters
        let string_vec: Vec<char> = LEVEL.0
            .chars().filter(|a| *a != '\r' && *a != '\n')
            .collect();

        let mut i = 0;

        for y in 0 .. LEVEL.2
        {
            for x in 0 .. LEVEL.1
            {
                let idx = map_idx(x, y);
                let c = string_vec[i];

                match c
                {
                    'M' =>
                        {
                            mb.map.tiles[idx] = TileType::Floor;
                            mb.monster_spawns.push(Point::new(x, y));
                        }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", c)
                }

                i += 1;
            }
        }

        mb
    }
}