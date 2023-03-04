use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for x in camera.left_x..camera.right_x {
        for y in camera.top_y..camera.bottom_y {
            let point = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if let Some(point_idx) = map.try_point(point) {
                let glyph = match map.tiles[point_idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Hall => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(point - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}
