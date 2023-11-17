use crate::prelude::*;

// Implements the Map Rendering system
#[system]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn map_render(
    ecs: &SubWorld,
    #[resource] map: &Map,          // Returns a reference to the stored Map resource
    #[resource] camera: &Camera)    // Returns a reference to the stored Camera resource
{
    // Retrieve the FieldOfView component of the player
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Start a new drawing batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y .. camera.bottom_y
    {
        for x in camera.left_x .. camera.right_x
        {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            // Check if the current tile is within the map and if it is visible based on the current FieldOfView of the player
            if map.in_bounds(pt) && player_fov.visible_tiles.contains(&pt)
            {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx]
                {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#')
                };

                // Add a drawing command to the batch
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    // Submit the drawing batch
    draw_batch.submit(0).expect("Batch error");
}