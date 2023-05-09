use bendy2d::polygon::Polygon;
use bendy2d::solver::{Bounds, Solver};
use macroquad::prelude::*;
use nalgebra::Vector2;

#[macroquad::main("BasicShapes")]
async fn main() {
    let screen_size = Vector2::new(1920.0, 1080.0);
    request_new_screen_size(screen_size.x, screen_size.y);
    let scale = 30.0;

    let mut solver = Solver::new();
    solver.bounds = Bounds {
        pos: Vector2::new(0.0, 0.0),
        size: screen_size / scale,
    };

    let size = Vector2::new(100.0, 100.0);
    let pos1 = Vector2::new(200.0, 200.0);
    let pos2 = Vector2::new(1600.0, 200.0);
    solver.add_polygon(Polygon::new_box(
        pos1 / scale,
        0.0,
        size / scale,
        4.0,
        50.0,
        false,
    ));

    solver.add_polygon(Polygon::new_box(
        pos2 / scale,
        0.0,
        size / scale,
        4.0,
        50.0,
        false,
    ));

    let polygon1 = solver.get_polygon_mut(0).unwrap();
    for particle in polygon1.particles.iter_mut() {
        particle.prev_pos = particle.pos - Vector2::new(10.0, 0.0);
    }
    let polygon2 = solver.get_polygon_mut(1).unwrap();
    for particle in polygon2.particles.iter_mut() {
        particle.prev_pos = particle.pos + Vector2::new(10.0, 0.0);
    }

    // Refresh window
    clear_background(RED);
    next_frame().await;
    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Space) || is_key_down(KeyCode::W) {
            solver.update(1.0 / 60.0);
        }

        for (i, polygon) in solver.get_polygons().iter().enumerate() {
            let particle_count = polygon.particles.len();
            for (j, point) in polygon.particles.iter().enumerate() {
                draw_triangle(
                    Vec2::new(point.pos.x, point.pos.y) * scale,
                    Vec2::new(
                        polygon.particles[(j + 1) % particle_count].pos.x,
                        polygon.particles[(j + 1) % particle_count].pos.y,
                    ) * scale,
                    Vec2::new(polygon.center.x, polygon.center.y) * scale,
                    Color::new(1.0, 1.0, i as f32, 1.0),
                );
            }
        }

        next_frame().await
    }
}
