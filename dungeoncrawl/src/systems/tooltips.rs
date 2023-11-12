use crate::prelude::*;

// Implements the ToolTip system
#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource]camera: &Camera)
{
    // Return all entities with a Point and Name component - Monsters in our case
    let mut positions = <(Entity, &Point, &Name)>::query();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos) // Returns only the elements where the position is the same as the current mouse position
        .for_each(|(entity, _, name)|           // Iterate over the elements
        {
            let screen_pos = *mouse_pos * 4;
            let display = if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>()
            {
                format!("{} : {} Hit Point", &name.0, health.current)
            }
            else
            {
                name.0.clone()
            };

            draw_batch.print_color(screen_pos, &display,  ColorPair::new(YELLOW, BLACK));
        });

    draw_batch.submit(10100).expect("Batch error");
}