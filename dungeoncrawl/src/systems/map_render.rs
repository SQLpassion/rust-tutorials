use crate::prelude::*;

// Implements the Map Rendering system
#[system]
pub fn map_render(
    #[resource] map: &Map,          // Returns a reference to the stored Map resource
    #[resource] camera: &Camera)    // Returns a reference to the stored Camera resource
{
    // Start a new drawing batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y .. camera.bottom_y
    {
        for x in camera.left_x .. camera.right_x
        {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if map.in_bounds(pt)
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