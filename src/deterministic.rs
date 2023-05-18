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

    let mut solver = Solver::new();
    solver.gravity = Vector2::new(0.0, 980.0);
    solver.bounds = Bounds {
        pos: Vector2::new(0.0, 0.0),
        size: screen_size,
    };

    let size = 15.0;
    let pos = Vector2::new(100.0, 100.0);
    for x in 0..3 {
        for y in 0..3 {
            let index = Vector2::new(x as f32, y as f32);
            solver.add_polygon(Polygon::circle(
                size,
                pos + Vector2::new(index.x * size * 2.0, index.y * size * 2.0),
                3,
                false,
                500.0,
                -1.0,
            ));
        }
    }

    let sim_steps = 3000;
    let mut positions = Vec::new();
    positions.resize(sim_steps, vec![Vector2::new(0.0, 0.0); 9 * 3]);
    let mut frame_count = 0;

    // Refresh window
    clear_background(RED);
    next_frame().await;
    loop {
        if frame_count >= sim_steps {
            break;
        }

        clear_background(BLACK);

        solver.update(0.005);

        for (i, polygon) in solver.get_polygons().iter().enumerate() {
            let particle_count = polygon.particles.len();
            for (j, point) in polygon.particles.iter().enumerate() {
                positions[frame_count][i * 3 + j] = point.pos;
                draw_triangle(
                    Vec2::new(point.pos.x, point.pos.y),
                    Vec2::new(
                        polygon.particles[(j + 1) % particle_count].pos.x,
                        polygon.particles[(j + 1) % particle_count].pos.y,
                    ),
                    Vec2::new(polygon.center.x, polygon.center.y),
                    WHITE,
                );
            }
        }

        frame_count += 1;
        next_frame().await
    }

    // Compare the positions to the previous run
    if Path::new("positions.csv").exists() {
        let mut file = File::open("positions.csv");
        if let Err(e) = file {
            println!("Error creating file: {}", e);
            return;
        }
        let mut file = file.unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data);
        let lines = data.split("\n");
        let mut failed_count = 0;
        for (i, latency) in positions.iter().enumerate() {
            let line = lines.clone().nth(i).unwrap();
            let points = line.split(",");
            for (j, point) in latency.iter().enumerate() {
                let x = points.clone().nth(j * 2).unwrap().parse::<f32>().unwrap();
                let y = points
                    .clone()
                    .nth(j * 2 + 1)
                    .unwrap()
                    .parse::<f32>()
                    .unwrap();
                if point.x != x || point.y != y {
                    println!("Mismatch at frame {}, point {}", i, j);
                    println!("Expected: {}, {}", x, y);
                    println!("Actual: {}, {}", point.x, point.y);
                    failed_count += 1;
                }
            }
        }
        if failed_count == 0 {
            println!("All points matched!");
        } else {
            println!("{} points did not match!", failed_count);
        }
        return;
    }
    // Write out the positions to a CSV file
    let mut file = File::create("positions.csv");
    if let Err(e) = file {
        println!("Error creating file: {}", e);
        return;
    }
    let mut file = file.unwrap();

    for latency in positions.iter() {
        for (i, point) in latency.iter().enumerate() {
            file.write(format!("{},{}", point.x, point.y).as_bytes());
            if i != latency.len() - 1 {
                file.write(",".as_bytes());
            }
        }
        file.write("\n".as_bytes());
    }
}
