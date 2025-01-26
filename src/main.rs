use macroquad::prelude::*;
mod camera;
use camera::Camera;

#[macroquad::main("Pacmanish")]
async fn main() {
    let mut camera = Camera::new(Vec2::ZERO, 0.5, Vec2::new(5., 5.));
    let map = Vec::from([
        1, 1, 1, 1, 1,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        1, 1, 1, 1, 1,  
    ]);


    loop {
        camera.show_view();
        for i in 0..map.len() {
            let cell = Vec2::new((i % 5) as f32, (i / 5) as f32);
            if map[i] == 1 {
                camera.draw_vec_rectangle(cell, Vec2::ONE, WHITE);
            }
        }
        camera.update(camera.position, 0.1);
        next_frame().await
    }
}
