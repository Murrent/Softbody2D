mod physics;

use crate::physics::circle::Circle;
use crate::physics::particle::Particle;
use crate::physics::solver::{Bounds, Solver};
use egui_macroquad::{egui, ui};
use macroquad::miniquad::fs::Response;
use macroquad::prelude::*;
use physics::scene::Scene;
use vector2d::Vector2D;
use crate::physics::link::{Link, ParticleLink};

fn spawn_particle_array(solver: &mut Solver, pos: &Vector2D<f32>, count: &Vector2D<u32>, dist: f32) {
    // Spawn particles in a grid and link neighbouring particles together
    for y in 0..count.y {
        for x in 0..count.x {
            let particle_pos = Vector2D {
                x: pos.x + x as f32 * dist,
                y: pos.y + y as f32 * dist,
            };
            solver.add_particle(&particle_pos);
            if x > 0 {
                solver.add_particle_link(ParticleLink {
                    link: Link {
                        particle_a: solver.get_particle_len() - 2,
                        particle_b: solver.get_particle_len() - 1,
                        target_distance: dist,
                    },
                });
            }
            if y > 0 {
                solver.add_particle_link(ParticleLink {
                    link: Link {
                        particle_a: solver.get_particle_len() - count.x as usize - 1,
                        particle_b: solver.get_particle_len() - 1,
                        target_distance: dist,
                    },
                });
                if x < count.x - 1 {
                    solver.add_particle_link(ParticleLink {
                        link: Link {
                            particle_a: solver.get_particle_len() - count.x as usize,
                            particle_b: solver.get_particle_len() - 1,
                            target_distance: (dist*dist+dist*dist).sqrt(),
                        },
                    });
                }
            }
            if x > 0 && y > 0 {
                solver.add_particle_link(ParticleLink {
                    link: Link {
                        particle_a: solver.get_particle_len() - count.x as usize - 2,
                        particle_b: solver.get_particle_len() - 1,
                        target_distance: (dist*dist+dist*dist).sqrt(),
                    },
                });
            }
        }
    }
}
fn update(solver: &mut Solver, dt: &f32, radius: &mut f32) {
    let mouse_pos: Vector2D<f32> = mouse_position().into();

    if is_mouse_button_pressed(MouseButton::Left) {
        spawn_particle_array(solver, &mouse_pos, &Vector2D { x: 10, y: 3 }, 25.0);
    } else if is_mouse_button_pressed(MouseButton::Right) {
        for x in 0..10 {
            for y in 0..10 {
                let spawn_pos = Vector2D {
                    x: mouse_pos.x + x as f32 * *radius * 2.0,
                    y: mouse_pos.y + y as f32 * *radius * 2.0,
                };
                solver.add_circle(Circle {
                    point: Particle::new(&spawn_pos),
                    radius: *radius,
                });
            }
        }
    } else if mouse_wheel().1 > 0.0 {
        *radius += 1.0;
    } else if mouse_wheel().1 < 0.0 {
        *radius -= 1.0;
    }

    solver.update(dt);
}
#[macroquad::main("BasicShapes")]
async fn main() {
    rand::srand(get_time() as u64);

    let mut solver = Solver::new();
    solver.bounds.size = Vector2D {
        x: screen_width(),
        y: screen_height(),
    };
    solver.gravity = Vector2D::new(0.0, 100000.0);

    let mut radius = 10.0;

    loop {
        clear_background(RED);

        let dt = get_frame_time();
        let mouse_pos: Vector2D<f32> = mouse_position().into();

        if dt < 0.1 {
            update(&mut solver, &dt, &mut radius);
        }

        let particle_count;
        let circle_count;

        {
            let particles = solver.get_particles();
            for particle in particles.iter() {
                draw_circle(particle.pos.x, particle.pos.y, 1.0, WHITE);
                //particle.add_force_towards(&mouse_pos, &100.0);
            }
            particle_count = particles.len();

            let links = solver.get_particle_links();
            for link in links.iter() {
                let particle_a = particles[link.link.particle_a];
                let particle_b = particles[link.link.particle_b];
                draw_line(
                    particle_a.pos.x,
                    particle_a.pos.y,
                    particle_b.pos.x,
                    particle_b.pos.y,
                    1.0,
                    GREEN,
                );
            }
        }

        {
            let circles = solver.get_circles();
            for circle in circles.iter() {
                draw_circle(circle.point.pos.x, circle.point.pos.y, circle.radius, BLUE);
            }
            circle_count = circles.len();
        }


        ui(|egui_ctx| {
            egui::Window::new("Information").show(egui_ctx, |ui| {
                ui.label(format!("FPS: {}", get_fps().to_string().as_str()));
                ui.label(format!("ms: {}", dt));
                ui.label(format!("Mouse: {} {}", mouse_pos.x, mouse_pos.y));
                ui.label(format!("Radius: {}", radius));
                ui.label(format!("Particles: {}", particle_count));
                ui.label(format!("Circles: {}", circle_count));
                if ui.button("Click me").clicked() {
                    solver = Solver::new();
                    solver.bounds.size = Vector2D {
                        x: screen_width(),
                        y: screen_height(),
                    };
                    solver.gravity = Vector2D::new(0.0, 100000.0);
                }
            });
        });
        egui_macroquad::draw();
        next_frame().await
    }
}
