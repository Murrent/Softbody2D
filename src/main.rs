mod physics;

use physics::scene::Scene;
use egui_macroquad::egui;
use macroquad::prelude::*;
use vector2d::Vector2D;
use crate::physics::solver::{Bounds, Solver};
use crate::physics::circle::Circle;
use crate::physics::particle::Particle;

#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(get_time() as u64);

    let mut solver = Solver::new();
    solver.bounds.size = Vector2D { x: screen_width(), y: screen_height() };
    solver.add_particle(&Vector2D { x: 500.0, y: 100.0 });
    solver.add_circle(Circle {
        point: Particle::new(&Vector2D { x: 400.0, y: 100.0 }),
        radius: 10.0,
    });
    loop {
        clear_background(RED);

        let dt = get_frame_time();
        let mouse_pos: Vector2D<f32> = mouse_position().into();

        solver.update(&dt);

        {
            let particles = solver.get_particles();
            for particle in particles.iter_mut() {
                draw_circle(particle.pos.x, particle.pos.y, 10.0, WHITE);
                particle.add_force_towards(&mouse_pos, &100.0);
            }
        }

        {
            let circles = solver.get_circles();
            for circle in circles.iter_mut() {
                draw_circle(circle.point.pos.x, circle.point.pos.y, circle.radius, BLUE);
                circle.point.add_force_towards(&mouse_pos, &100.0);
            }
        }

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
                ui.label(format!("FPS: {}", get_fps().to_string().as_str()));
                ui.label(format!("ms: {}", dt));
                ui.label(format!("Mouse: {} {}", mouse_pos.x, mouse_pos.y));
            });
        });
        egui_macroquad::draw();
        next_frame().await
    }
}
