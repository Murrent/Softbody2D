use bendy2d::circle::Circle;
use bendy2d::link::{CircleLink, Link, ParticleLink};
use bendy2d::particle::Particle;
use bendy2d::polygon::Polygon;
use bendy2d::solver::Solver;
use egui_macroquad::egui::Pos2;
use egui_macroquad::{egui, ui};
use macroquad::math::{f32, u32};
use macroquad::prelude::*;
use nalgebra::Vector2;

enum SpawnMode {
    Single,
    Grid,
    Last,
    Spam,
}

fn spawn_mode_string(spawn_mode: &SpawnMode) -> String {
    match spawn_mode {
        SpawnMode::Single => "Single".to_string(),
        SpawnMode::Grid => "Grid".to_string(),
        SpawnMode::Last => "Last".to_string(),
        SpawnMode::Spam => "Spam".to_string(),
    }
}

fn spawn_particle_array(solver: &mut Solver, pos: Vector2<f32>, count: Vector2<u32>, dist: f32) {
    for y in 0..count.y {
        for x in 0..count.x {
            let particle_pos = Vector2::new(
                pos.x + x as f32 * dist,
                pos.y + y as f32 * dist,
            );
            solver.add_particle(particle_pos);
            let length = solver.get_particle_len();
            if x > 0 {
                solver.add_particle_link(ParticleLink {
                    link: Link {
                        particle_a: length - 2,
                        particle_b: length - 1,
                        target_distance: dist,
                    },
                });
            }
            if y > 0 {
                solver.add_particle_link(ParticleLink {
                    link: Link {
                        particle_a: length - count.x as usize - 1,
                        particle_b: length - 1,
                        target_distance: dist,
                    },
                });
                if x < count.x - 1 {
                    solver.add_particle_link(ParticleLink {
                        link: Link {
                            particle_a: length - count.x as usize,
                            particle_b: length - 1,
                            target_distance: (dist * dist + dist * dist).sqrt(),
                        },
                    });
                }
            }
            if x > 0 && y > 0 {
                solver.add_particle_link(ParticleLink {
                    link: Link {
                        particle_a: length - count.x as usize - 2,
                        particle_b: length - 1,
                        target_distance: (dist * dist + dist * dist).sqrt(),
                    },
                });
            }
        }
    }
}

fn spawn_circle_array(
    solver: &mut Solver,
    pos: Vector2<f32>,
    count: Vector2<u32>,
    dist: f32,
    radius: f32,
) {
    for y in 0..count.y {
        for x in 0..count.x {
            let circle_pos = Vector2::new(
                pos.x + x as f32 * dist,
                pos.y + y as f32 * dist,
            );
            solver.add_circle(Circle {
                point: Particle::new(circle_pos),
                radius,
            });
            let length = solver.get_circles_len();
            if x > 0 {
                solver.add_circle_link(CircleLink {
                    link: Link {
                        particle_a: length - 2,
                        particle_b: length - 1,
                        target_distance: dist,
                    },
                });
            }
            if y > 0 {
                solver.add_circle_link(CircleLink {
                    link: Link {
                        particle_a: length - count.x as usize - 1,
                        particle_b: length - 1,
                        target_distance: dist,
                    },
                });
                if x < count.x - 1 {
                    solver.add_circle_link(CircleLink {
                        link: Link {
                            particle_a: length - count.x as usize,
                            particle_b: length - 1,
                            target_distance: (dist * dist + dist * dist).sqrt(),
                        },
                    });
                }
            }
            if x > 0 && y > 0 {
                solver.add_circle_link(CircleLink {
                    link: Link {
                        particle_a: length - count.x as usize - 2,
                        particle_b: length - 1,
                        target_distance: (dist * dist + dist * dist).sqrt(),
                    },
                });
            }
        }
    }
}

fn input_single(solver: &mut Solver, radius: f32, mouse_pos: Vector2<f32>) {
    if is_mouse_button_pressed(MouseButton::Left) {
        solver.add_particle(mouse_pos);
    } else if is_mouse_button_pressed(MouseButton::Right) {
        solver.add_circle(Circle {
            point: Particle::new(mouse_pos),
            radius,
        });
    } else if is_mouse_button_pressed(MouseButton::Middle) {
        solver.add_polygon(Polygon::circle(radius, mouse_pos, 10, false));
    }
}

fn input_grid(solver: &mut Solver, radius: f32, mouse_pos: Vector2<f32>) {
    if is_mouse_button_pressed(MouseButton::Left) {
        spawn_particle_array(solver, mouse_pos, Vector2::new(5, 5), radius);
    } else if is_mouse_button_pressed(MouseButton::Right) {
        spawn_circle_array(
            solver,
            mouse_pos,
            Vector2::new(5, 5),
            radius * 2.0,
            radius,
        )
    } else if is_mouse_button_pressed(MouseButton::Middle) {
        solver.add_polygon(Polygon::new(vec![
            mouse_pos + Vector2::new(-radius, -radius),
            mouse_pos + Vector2::new(radius, -radius),
            mouse_pos + Vector2::new(radius, radius),
        ], false));
    }
}

fn input_last(solver: &mut Solver, radius: f32, mouse_pos: Vector2<f32>) {
    if is_mouse_button_pressed(MouseButton::Left) {
        solver.add_particle(mouse_pos);
        let length = solver.get_particle_len();
        if length < 2 {
            return;
        }
        if let Some(particle) = solver.get_particle(length - 2) {
            solver.add_particle_link(ParticleLink {
                link: Link {
                    particle_a: length - 2,
                    particle_b: length - 1,
                    target_distance: (mouse_pos - particle.pos).magnitude(),
                },
            });
        }
    } else if is_mouse_button_pressed(MouseButton::Right) {
        solver.add_circle(Circle {
            point: Particle::new(mouse_pos),
            radius,
        });
        let length = solver.get_circles_len();
        if length < 2 {
            return;
        }
        if let Some(particle) = solver.get_circle(length - 2) {
            solver.add_circle_link(CircleLink {
                link: Link {
                    particle_a: length - 2,
                    particle_b: length - 1,
                    target_distance: (mouse_pos - particle.point.pos).magnitude(),
                },
            });
        }
    }
}

fn input_spam(solver: &mut Solver, radius: f32, mouse_pos: Vector2<f32>) {
    if is_mouse_button_down(MouseButton::Left) {
        solver.add_particle(mouse_pos);
    } else if is_mouse_button_down(MouseButton::Right) {
        solver.add_circle(Circle {
            point: Particle::new(mouse_pos),
            radius,
        });
    }
}

fn handle_input(solver: &mut Solver, radius: f32, spawn_mode: &SpawnMode, mouse_pos: Vector2<f32>) {
    match spawn_mode {
        SpawnMode::Single => input_single(solver, radius, mouse_pos),
        SpawnMode::Grid => input_grid(solver, radius, mouse_pos),
        SpawnMode::Last => input_last(solver, radius, mouse_pos),
        SpawnMode::Spam => input_spam(solver, radius, mouse_pos),
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(1280.0, 720.0);
    rand::srand(get_time() as u64);

    // Refresh window
    clear_background(RED);
    next_frame().await;

    let mut solver = Solver::new();
    solver.bounds.size = Vector2::new(
        screen_width(),
        screen_height(),
    );
    solver.gravity = Vector2::new(0.0, 100000.0);

    let mut radius = 10.0;
    let mut spawn_mode = SpawnMode::Single;
    let mut ui_hovered = false;
    let mut pause = false;
    let mut last_update = get_time();
    loop {
        // if get_time() - last_update < 0.1 {
        //     continue;
        // }
        // last_update = get_time();
        clear_background(BLACK);

        let dt = 0.005;//get_frame_time();
        let mouse_pos: Vector2<f32>;
        {
            let _mouse_pos = mouse_position();
            mouse_pos = Vector2::<f32>::new(_mouse_pos.0, _mouse_pos.1);
        }

        if dt < 0.1 {
            if !ui_hovered {
                if mouse_wheel().1 > 0.0 {
                    radius += 1.0;
                } else if mouse_wheel().1 < 0.0 {
                    radius -= 1.0;
                }
                handle_input(&mut solver, radius, &spawn_mode, mouse_pos);
            }

            if !pause {
                solver.update(dt);
            }
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

            let links = solver.get_circle_links();
            for link in links.iter() {
                let particle_a = circles[link.link.particle_a];
                let particle_b = circles[link.link.particle_b];
                draw_line(
                    particle_a.point.pos.x,
                    particle_a.point.pos.y,
                    particle_b.point.pos.x,
                    particle_b.point.pos.y,
                    1.0,
                    YELLOW,
                );
            }
        }

        {
            let polygons = solver.get_polygons();
            for polygon in polygons.iter() {
                let points = &polygon.points;
                for (i, point) in points.iter().enumerate() {
                    let point_b = points.get((i + 1) % points.len());
                    if let Some(point_b) = point_b {
                        draw_line(point.pos.x, point.pos.y, point_b.pos.x, point_b.pos.y, 1.0, GREEN);
                    }
                    draw_circle(point.pos.x, point.pos.y, 1.0, GREEN);
                }

                // for (i, link) in polygon.particle_links.iter().enumerate() {
                //     draw_line(
                //         points[link.link.particle_a].pos.x,
                //         points[link.link.particle_a].pos.y,
                //         points[link.link.particle_b].pos.x,
                //         points[link.link.particle_b].pos.y,
                //         1.0,
                //         WHITE,
                //     );
                //     // draw_text(
                //     //     format!("{}", i).as_str(),
                //     //     points[link.link.particle_a].pos.x,
                //     //     points[link.link.particle_a].pos.y,
                //     //     28.0,
                //     //     WHITE,
                //     // );
                // }


                draw_circle(polygon.center.x, polygon.center.y, 1.0, WHITE);
            }
        }

        ui(|egui_ctx| {
            let hovered = egui::Window::new("Information")
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps().to_string().as_str()));
                    ui.label(format!("ms: {}", dt));
                    ui.label(format!("Mouse: {} {}", mouse_pos.x, mouse_pos.y));
                    ui.label(format!("Radius: {}", radius));
                    ui.label(format!("Particles: {}", particle_count));
                    ui.label(format!("Circles: {}", circle_count));
                    if ui.button("Reset").clicked() {
                        solver = Solver::new();
                        solver.bounds.size = Vector2::new(
                            screen_width(),
                            screen_height(),
                        );
                        solver.gravity = Vector2::new(0.0, 100000.0);
                    }

                    ui.label(format!("Spawn mode: {}", spawn_mode_string(&spawn_mode)));
                    if ui.button("Change mode").clicked() {
                        match spawn_mode {
                            SpawnMode::Single => spawn_mode = SpawnMode::Grid,
                            SpawnMode::Grid => spawn_mode = SpawnMode::Last,
                            SpawnMode::Last => spawn_mode = SpawnMode::Spam,
                            SpawnMode::Spam => spawn_mode = SpawnMode::Single,
                        }
                    }
                    if ui
                        .button(match pause {
                            true => "Resume",
                            false => "Pause",
                        })
                        .clicked()
                    {
                        pause = !pause;
                    }
                })
                .unwrap()
                .response
                .rect
                .contains(Pos2 {
                    x: mouse_pos.x,
                    y: mouse_pos.y,
                });
            ui_hovered = hovered;
        });

        egui_macroquad::draw();
        next_frame().await
    }
}
