use crate::prelude::*;

// Implements the Entity rendering system
#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(
    ecs: &SubWorld, 
    #[resource] camera: &Camera) // Returns a reference to the stored Camera resource
{
    // Retrieve all the entities with a Point and Render component
    let mut renderables = <(&Point, &Render)>::query();

    // Retrieve the FieldOfView component of the player
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Create a new batch for drawing
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // Render each entity
    renderables
        .iter(ecs)
        // Render the entity only if it is visible based on the current FieldOfView of the player
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        .for_each(|(pos, render)|
        {
            // Render the entity
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // Submit the batch for drawing
    draw_batch.submit(5000).expect("Batch error");
}