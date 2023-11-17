use crate::prelude::*;

// Implements the ToolTip system
#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource]camera: &Camera)
{
    // Return all entities with a Point and Name component - Monsters in our case
    let mut positions = <(Entity, &Point, &Name)>::query();
    
    // Retrieve the FieldOfView component of the player
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    // Create a new batch for drawing
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    positions
        .iter(ecs)
        // Returns only the elements where the position is the same as the current mouse position,
        // and if the tile is currently visible based on the players FOV
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos)) 
        // Iterate over the elements
        .for_each(|(entity, _, name)|
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

    // Submit the batch for drawing
    draw_batch.submit(10100).expect("Batch error");
}