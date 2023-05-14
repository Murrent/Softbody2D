use std::time::Instant;
use bendy2d::polygon::Polygon;
use bendy2d::solver::{Bounds, Solver};
use macroquad::prelude::*;
use nalgebra::Vector2;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
    solver.gravity = Vector2::new(0.0, 0.98);

    let size = Vector2::new(32.0, 32.0);
    let pos = Vector2::new(100.0, 100.0);
    for x in 0..15 {
        for y in 0..15 {
            let index = Vector2::new(x as f32, y as f32);
            solver.add_polygon(Polygon::new_box(
                (pos + Vector2::new(index.x + index.x * size.x, index.y + index.y * size.y)) / scale,
                0.0,
                size / scale,
                4.0,
                200.0,
                false,
            ));
        }
    }

    let sim_steps = 2000;
    let mut latencies = Vec::<f32>::new();
    latencies.resize(sim_steps, 0.0);
    let clock = Instant::now();
    let mut frame_count = 0;

    // Refresh window
    clear_background(RED);
    next_frame().await;
    loop {
        if frame_count >= sim_steps {
            break;
        }

        clear_background(BLACK);

        let last_frame_time = clock.elapsed().as_secs_f32();

        solver.update(1.0 / 60.0);

        latencies[frame_count] = clock.elapsed().as_secs_f32() - last_frame_time;
        frame_count += 1;

        for polygon in solver.get_polygons().iter() {
            let particle_count = polygon.particles.len();
            for (i, point) in polygon.particles.iter().enumerate() {
                draw_triangle(
                    Vec2::new(point.pos.x, point.pos.y) * scale,
                    Vec2::new(
                        polygon.particles[(i + 1) % particle_count].pos.x,
                        polygon.particles[(i + 1) % particle_count].pos.y,
                    ) * scale,
                    Vec2::new(polygon.center.x, polygon.center.y) * scale,
                    WHITE,
                );
            }
        }

        next_frame().await
    }
    for latency in latencies.iter() {
        println!("{}", latency);
    }

    // Write out the positions to a CSV file
    let mut file = File::create("performance1.csv");
    if let Err(e) = file {
        println!("Error creating file: {}", e);
        return;
    }
    let mut file = file.unwrap();

    let mut data = String::new();
    for latency in latencies.iter() {
        data += &format!("{}\n", latency);
    }
    file.write_all(data.as_bytes()).expect("Failed to write to file");
}
