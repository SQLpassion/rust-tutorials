use crate::prelude::*;

// Implements the Entity rendering system
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(
    ecs: &SubWorld, 
    #[resource] camera: &Camera) // Returns a reference to the stored Camera resource
{
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // Return all the entities with a Point and Render component
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)|
        {
            // Render the entity
            draw_batch.set(*pos - offset, render.color, render.glyph);
        }
    );

    draw_batch.submit(5000).expect("Batch error");
}