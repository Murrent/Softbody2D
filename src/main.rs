use bendy2d::circle::Circle;

use bendy2d::link::{CircleLink, Link, ParticleLink};
use bendy2d::particle::Particle;
use bendy2d::polygon::{Collision, Polygon};
use bendy2d::solver::Solver;
use bendy2d::spring::Spring;
use egui_macroquad::egui::Pos2;
use egui_macroquad::{egui, ui};
use macroquad::math::{f32, u32};
use macroquad::prelude::*;
use nalgebra::Vector2;

// TODO: remove this
pub enum CollisionPhase {
    Points,
    RealIntersection,
    Influence,
    Penetration,
    Displacement,
    NewPoints,
}

impl CollisionPhase {
    fn name(&self) -> &str {
        match *self {
            CollisionPhase::Points => "PointA",
            CollisionPhase::RealIntersection => "RealIntersection",
            CollisionPhase::Influence => "Influence",
            CollisionPhase::Penetration => "PenOnNormal",
            CollisionPhase::Displacement => "DisplacementB",
            CollisionPhase::NewPoints => "NewPoint",
        }
    }

    fn increase(&mut self) {
        *self = match *self {
            CollisionPhase::Points => CollisionPhase::RealIntersection,
            CollisionPhase::RealIntersection => CollisionPhase::Influence,
            CollisionPhase::Influence => CollisionPhase::Penetration,
            CollisionPhase::Penetration => CollisionPhase::Displacement,
            CollisionPhase::Displacement => CollisionPhase::NewPoints,
            CollisionPhase::NewPoints => CollisionPhase::Points,
        }
    }

    fn decrease(&mut self) {
        *self = match *self {
            CollisionPhase::Points => CollisionPhase::NewPoints,
            CollisionPhase::RealIntersection => CollisionPhase::Points,
            CollisionPhase::Influence => CollisionPhase::RealIntersection,
            CollisionPhase::Penetration => CollisionPhase::Influence,
            CollisionPhase::Displacement => CollisionPhase::Penetration,
            CollisionPhase::NewPoints => CollisionPhase::Displacement,
        }
    }

    fn draw(&self, collision: &Collision, polygons: &[Polygon]) {
        let size_large = 50.0;
        let size_small = 30.0;
        let dot_size = 5.0;
        let line_width = 3.0;

        draw_line(
            collision.point_a.pos.x,
            collision.point_a.pos.y,
            collision.point_b.pos.x,
            collision.point_b.pos.y,
            line_width,
            YELLOW,
        );
        draw_circle(
            collision.point.pos.x,
            collision.point.pos.y,
            dot_size,
            YELLOW,
        );
        draw_line(
            collision.new_a.x,
            collision.new_a.y,
            collision.new_b.x,
            collision.new_b.y,
            line_width,
            GREEN,
        );
        draw_circle(
            collision.new_point.x,
            collision.new_point.y,
            dot_size,
            GREEN,
        );

        match *self {
            CollisionPhase::Points => {
                draw_text("Points", 30.0, 30.0, size_large, WHITE);
                draw_circle(
                    collision.point_a.pos.x,
                    collision.point_a.pos.y,
                    dot_size,
                    WHITE,
                );
                draw_circle(
                    collision.point_b.pos.x,
                    collision.point_b.pos.y,
                    dot_size,
                    WHITE,
                );
                draw_text(
                    "The point of the line".to_string().as_str(),
                    collision.point_a.pos.x,
                    collision.point_a.pos.y,
                    size_small,
                    WHITE,
                );

                draw_text(
                    "The point that intersects".to_string().as_str(),
                    collision.point.pos.x,
                    collision.point.pos.y,
                    size_small,
                    BLUE,
                );
                draw_circle(collision.point.pos.x, collision.point.pos.y, dot_size, BLUE);
            }
            CollisionPhase::RealIntersection => {
                draw_text("Real Intersection", 30.0, 30.0, size_large, WHITE);
                // draw_text(
                //     format!(
                //         "Projecting the center onto the line: {}, {}",
                //         collision.center_proj.x, collision.center_proj.y
                //     )
                //     .as_str(),
                //     collision.intersection.x + collision.center_proj.x / 2.0 + 200.0,
                //     collision.intersection.y + collision.center_proj.y / 2.0 - 200.0,
                //     size_small,
                //     WHITE,
                // );
                // draw_line(
                //     collision.intersection.x + collision.center_proj.x / 2.0,
                //     collision.intersection.y + collision.center_proj.y / 2.0,
                //     collision.intersection.x + collision.center_proj.x / 2.0 + 200.0,
                //     collision.intersection.y + collision.center_proj.y / 2.0 - 200.0,
                //     line_width,
                //     DARKBROWN,
                // );
                // draw_line(
                //     collision.intersection.x,
                //     collision.intersection.y,
                //     collision.intersection.x + collision.center_proj.x,
                //     collision.intersection.y + collision.center_proj.y,
                //     line_width,
                //     WHITE,
                // );
                // draw_text(
                //     "The normal vector inwards towards the center"
                //         .to_string()
                //         .as_str(),
                //     collision.center.x,
                //     collision.center.y,
                //     size_small,
                //     WHITE,
                // );
                // draw_line(
                //     collision.intersection.x + collision.center_proj.x,
                //     collision.intersection.y + collision.center_proj.y,
                //     collision.center.x,
                //     collision.center.y,
                //     line_width,
                //     WHITE,
                // );
                // draw_text(
                //     "Projecting the point onto the line".to_string().as_str(),
                //     collision.intersection.x + collision.point_proj.x,
                //     collision.intersection.y + collision.point_proj.y,
                //     size_small,
                //     WHITE,
                // );
                // draw_line(
                //     collision.intersection.x,
                //     collision.intersection.y,
                //     collision.intersection.x + collision.point_proj.x,
                //     collision.intersection.y + collision.point_proj.y,
                //     line_width,
                //     WHITE,
                // );
                // draw_circle(
                //     collision.real_intersection.x,
                //     collision.real_intersection.y,
                //     dot_size,
                //     WHITE,
                // );
            }
            CollisionPhase::Influence => {
                draw_text(
                    format!(
                        "Distance to A: {} / {} = {}",
                        collision.dist_to_b, collision.dist_a_to_b, collision.influence_a
                    )
                    .as_str(),
                    collision.point_a.pos.x,
                    collision.point_a.pos.y,
                    size_small,
                    BLUE,
                );
                // draw_line(
                //     collision.point_a.pos.x,
                //     collision.point_a.pos.y,
                //     collision.point_a.pos.x + collision.line_normalized.x * collision.dist_to_a,
                //     collision.point_a.pos.y + collision.line_normalized.y * collision.dist_to_a,
                //     line_width,
                //     BLUE,
                // );
                draw_text(
                    format!(
                        "Distance to B: {} / {} = {}",
                        collision.dist_to_a, collision.dist_a_to_b, collision.influence_b
                    )
                    .as_str(),
                    collision.point_b.pos.x,
                    collision.point_b.pos.y,
                    size_small,
                    WHITE,
                );
                // draw_line(
                //     collision.point_b.pos.x,
                //     collision.point_b.pos.y,
                //     collision.point_b.pos.x - collision.line_normalized.x * collision.dist_to_b,
                //     collision.point_b.pos.y - collision.line_normalized.y * collision.dist_to_b,
                //     line_width,
                //     WHITE,
                // );
                draw_circle(
                    collision.point_a.pos.x,
                    collision.point_a.pos.y,
                    dot_size,
                    BLUE,
                );
                draw_circle(
                    collision.point_b.pos.x,
                    collision.point_b.pos.y,
                    dot_size,
                    WHITE,
                );
            }
            CollisionPhase::Penetration => {
                // draw_text(
                //     format!(
                //         "Penetration Vector: {}, {}",
                //         collision.pen_vector.x, collision.pen_vector.y,
                //     )
                //     .as_str(),
                //     collision.real_intersection.x,
                //     collision.real_intersection.y,
                //     size_small,
                //     BLUE,
                // );
                // draw_line(
                //     collision.real_intersection.x,
                //     collision.real_intersection.y,
                //     collision.real_intersection.x - collision.pen_vector.x,
                //     collision.real_intersection.y - collision.pen_vector.y,
                //     line_width * 2.0,
                //     BLUE,
                // );
                // let pen_normalized = collision.pen_vector.normalize();
                // draw_line(
                //     collision.real_intersection.x,
                //     collision.real_intersection.y,
                //     collision.real_intersection.x - pen_normalized.x * 100.0,
                //     collision.real_intersection.y - pen_normalized.y * 100.0,
                //     line_width,
                //     WHITE,
                // );
            }
            CollisionPhase::Displacement => {
                // draw_text(
                //     "total_displacement = pen_on_normal / (a_inv_mass + b_inv_mass + c_inv_mass)"
                //         .to_string()
                //         .as_str(),
                //     collision.real_intersection.x,
                //     collision.real_intersection.y - size_small,
                //     size_small,
                //     WHITE,
                // );
                draw_text(
                    "Displace Point",
                    collision.point.pos.x + collision.displace_point.x,
                    collision.point.pos.y + collision.displace_point.y,
                    size_small,
                    WHITE,
                );
                draw_line(
                    collision.point.pos.x,
                    collision.point.pos.y,
                    collision.point.pos.x + collision.displace_point.x,
                    collision.point.pos.y + collision.displace_point.y,
                    line_width,
                    WHITE,
                );
                draw_circle(
                    collision.point.pos.x + collision.displace_point.x,
                    collision.point.pos.y + collision.displace_point.y,
                    dot_size,
                    WHITE,
                );
                draw_text(
                    "Displacement A",
                    collision.point_a.pos.x + collision.displacement_a.x,
                    collision.point_a.pos.y + collision.displacement_a.y,
                    size_small,
                    WHITE,
                );
                draw_line(
                    collision.point_a.pos.x,
                    collision.point_a.pos.y,
                    collision.point_a.pos.x + collision.displacement_a.x,
                    collision.point_a.pos.y + collision.displacement_a.y,
                    line_width,
                    WHITE,
                );
                draw_circle(
                    collision.point_a.pos.x + collision.displacement_a.x,
                    collision.point_a.pos.y + collision.displacement_a.y,
                    dot_size,
                    BLUE,
                );
                draw_text(
                    "Displacement B",
                    collision.point_b.pos.x + collision.displacement_b.x,
                    collision.point_b.pos.y + collision.displacement_b.y,
                    size_small,
                    WHITE,
                );
                draw_line(
                    collision.point_b.pos.x,
                    collision.point_b.pos.y,
                    collision.point_b.pos.x + collision.displacement_b.x,
                    collision.point_b.pos.y + collision.displacement_b.y,
                    line_width,
                    WHITE,
                );
                draw_circle(
                    collision.point_b.pos.x + collision.displacement_b.x,
                    collision.point_b.pos.y + collision.displacement_b.y,
                    dot_size,
                    BLUE,
                );
            }
            CollisionPhase::NewPoints => {
                draw_text("New A", 30.0, 30.0, size_large, WHITE);
                draw_circle(collision.new_a.x, collision.new_a.y, dot_size, WHITE);
                draw_text("New B", 30.0, 30.0, size_large, WHITE);
                draw_circle(collision.new_b.x, collision.new_b.y, dot_size, WHITE);
                draw_text("New Point", 30.0, 30.0, size_large, WHITE);
                draw_circle(
                    collision.new_point.x,
                    collision.new_point.y,
                    dot_size,
                    WHITE,
                )
            }
        }
    }

    fn is_last(&self) -> bool {
        matches!(*self, CollisionPhase::NewPoints)
    }
}

enum SpawnMode {
    Single,
    Grid,
    Last,
    Spam,
}

impl SpawnMode {
    fn name(&self) -> &str {
        match *self {
            SpawnMode::Single => "Single",
            SpawnMode::Grid => "Grid",
            SpawnMode::Last => "Last",
            SpawnMode::Spam => "Spam",
        }
    }

    fn increase(&mut self) {
        *self = match *self {
            SpawnMode::Single => SpawnMode::Grid,
            SpawnMode::Grid => SpawnMode::Last,
            SpawnMode::Last => SpawnMode::Spam,
            SpawnMode::Spam => SpawnMode::Single,
        }
    }
}

enum SpawnType {
    Particle,
    Circle,
    Polygon,
    PressurePolygon,
    Static,
}

impl SpawnType {
    fn name(&self) -> &str {
        match *self {
            SpawnType::Particle => "Particle",
            SpawnType::Circle => "Circle",
            SpawnType::Polygon => "Polygon",
            SpawnType::PressurePolygon => "PressurePolygon",
            SpawnType::Static => "Static",
        }
    }

    fn increase(&mut self) {
        *self = match *self {
            SpawnType::Particle => SpawnType::Circle,
            SpawnType::Circle => SpawnType::Polygon,
            SpawnType::Polygon => SpawnType::PressurePolygon,
            SpawnType::PressurePolygon => SpawnType::Static,
            SpawnType::Static => SpawnType::Particle,
        }
    }
}

enum TestCase {
    Playground,
    Triangle1,
    Triangle2,
    Circle1,
    Circle2,
}

impl TestCase {
    fn name(&self) -> &str {
        match *self {
            TestCase::Playground => "Playground",
            TestCase::Triangle1 => "Triangle1",
            TestCase::Triangle2 => "Triangle2",
            TestCase::Circle1 => "Circle1",
            TestCase::Circle2 => "Circle2",
        }
    }
    fn increase(&mut self) {
        *self = match *self {
            TestCase::Playground => TestCase::Triangle1,
            TestCase::Triangle1 => TestCase::Triangle2,
            TestCase::Triangle2 => TestCase::Circle1,
            TestCase::Circle1 => TestCase::Circle2,
            TestCase::Circle2 => TestCase::Playground,
        }
    }
}

fn spawn_particle_array(
    solver: &mut Solver,
    pos: Vector2<f32>,
    count: Vector2<u32>,
    dist: f32,
    stiffness: f32,
    permanence_threshold: f32,
) {
    for y in 0..count.y {
        for x in 0..count.x {
            let particle_pos = Vector2::new(pos.x + x as f32 * dist, pos.y + y as f32 * dist);
            solver.add_particle(particle_pos);
            let length = solver.get_particle_len();
            if x > 0 {
                solver.add_particle_spring(Spring {
                    particle_a: length - 2,
                    particle_b: length - 1,
                    rest_length: dist,
                    stiffness,
                    permanence_threshold,
                });
            }
            if y > 0 {
                solver.add_particle_spring(Spring {
                    particle_a: length - count.x as usize - 1,
                    particle_b: length - 1,
                    rest_length: dist,
                    stiffness,
                    permanence_threshold,
                });
                if x < count.x - 1 {
                    solver.add_particle_spring(Spring {
                        particle_a: length - count.x as usize,
                        particle_b: length - 1,
                        rest_length: (dist * dist + dist * dist).sqrt(),
                        stiffness,
                        permanence_threshold,
                    });
                }
            }
            if x > 0 && y > 0 {
                solver.add_particle_spring(Spring {
                    particle_a: length - count.x as usize - 2,
                    particle_b: length - 1,
                    rest_length: (dist * dist + dist * dist).sqrt(),
                    stiffness,
                    permanence_threshold,
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
            let circle_pos = Vector2::new(pos.x + x as f32 * dist, pos.y + y as f32 * dist);
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

struct Testbed {
    solver: Solver,
    gravity: Vector2<f32>,

    radius: f32,
    spawn_mode: SpawnMode,
    test_case: TestCase,
    spawn_type: SpawnType,
    point_count: usize,
    stiffness: f32,
    pressure: f32,
    permanence_threshold: f32,
    // Overlay vectors
    points_vec: Vec<Vector2<f32>>,
    circles_vec: Vec<Vector2<f32>>,
    links_vec: Vec<Vector2<usize>>,

    ui_hovered: bool,
    pause_on_collision: bool,
    pause: bool,
    step: bool,
    mouse_pos: Vector2<f32>,
    draw_aabb: bool,
    dt: f32,
    collision_phase: CollisionPhase,
    collision_index: usize,
    collisions: Vec<Collision>,
}

impl Testbed {
    fn new() -> Self {
        let mut solver = Solver::new();
        solver.bounds.size = Vector2::new(screen_width(), screen_height());
        let gravity = Vector2::new(0.0, 100.0);
        solver.gravity = gravity;

        let radius = 10.0;
        let spawn_mode = SpawnMode::Single;
        let test_case = TestCase::Playground;
        let spawn_type = SpawnType::Particle;
        let point_count = 5;
        let stiffness = 25.0;
        let points_vec = Vec::<Vector2<f32>>::new();
        let circles_vec = Vec::<Vector2<f32>>::new();
        let links_vec = Vec::<Vector2<usize>>::new();

        let ui_hovered = false;
        let pause_on_collision = false;
        let pause = false;
        let step = false;
        let mouse_pos = Vector2::<f32>::new(0.0, 0.0);
        let dt = 0.0;

        Self {
            solver,
            gravity,
            radius,
            spawn_mode,
            test_case,
            spawn_type,
            point_count,
            stiffness,
            pressure: 1.0,
            permanence_threshold: -1.0,
            points_vec,
            circles_vec,
            links_vec,
            ui_hovered,
            pause_on_collision,
            pause,
            step,
            mouse_pos,
            draw_aabb: false,
            dt,
            collision_phase: CollisionPhase::Points,
            collision_index: 0,
            collisions: Vec::<Collision>::new(),
        }
    }

    fn update(&mut self) {
        self.dt = 0.01; //get_frame_time();
        {
            let _mouse_pos = mouse_position();
            self.mouse_pos = Vector2::<f32>::new(_mouse_pos.0, _mouse_pos.1);
        }

        if self.dt < 0.1 {
            if !self.ui_hovered {
                if mouse_wheel().1 > 0.0 {
                    self.radius += 1.0;
                } else if mouse_wheel().1 < 0.0 {
                    self.radius -= 1.0;
                }
                self.overlay_clear();
                self.handle_input();
            }

            if !self.pause || self.step {
                self.step = false;
                self.solver.update(self.dt);
                self.collisions = self
                    .solver
                    .get_polygons()
                    .iter()
                    .flat_map(|p| &p.collisions)
                    .cloned()
                    .collect();
                if !self.collisions.is_empty() && self.pause_on_collision {
                    self.collision_phase = CollisionPhase::Points;
                    self.collision_index = 0;
                    self.pause = true;
                }
            } else {
                let collision = self.collisions.get(self.collision_index);
                if let Some(collision) = collision {
                    if is_key_pressed(KeyCode::Right) {
                        if self.collision_phase.is_last() {
                            self.collision_index += 1;
                        }
                        self.collision_phase.increase();
                    } else if is_key_pressed(KeyCode::Left) {
                        self.collision_phase.decrease();
                    }
                }
            }
        }
    }

    fn handle_input(&mut self) {
        match self.spawn_mode {
            SpawnMode::Single => self.input_single(),
            SpawnMode::Grid => self.input_grid(),
            SpawnMode::Last => self.input_last(),
            SpawnMode::Spam => self.input_spam(),
        }
    }

    fn input_single(&mut self) {
        let should_spawn = is_mouse_button_pressed(MouseButton::Left);
        match self.spawn_type {
            SpawnType::Particle => {
                self.overlay_particle();
                if should_spawn {
                    self.solver
                        .add_particle(self.mouse_pos + Vector2::new(self.radius, self.radius));
                    self.solver.add_particle(self.mouse_pos);
                    self.solver
                        .add_particle(self.mouse_pos + Vector2::new(0.0, self.radius));
                    let length = self.solver.get_particle_len();
                    self.solver.add_particle_spring(Spring {
                        particle_a: length - 3,
                        particle_b: length - 2,
                        rest_length: self.radius,
                        stiffness: self.stiffness,
                        permanence_threshold: self.permanence_threshold,
                    });
                    self.solver.add_particle_spring(Spring {
                        particle_a: length - 3,
                        particle_b: length - 1,
                        rest_length: self.radius,
                        stiffness: self.stiffness,
                        permanence_threshold: self.permanence_threshold,
                    });
                    self.solver.add_particle_spring(Spring {
                        particle_a: length - 2,
                        particle_b: length - 1,
                        rest_length: (self.radius * self.radius + self.radius * self.radius).sqrt(),
                        stiffness: self.stiffness,
                        permanence_threshold: self.permanence_threshold,
                    });
                }
            }
            SpawnType::Circle => {
                self.overlay_circle();
                if should_spawn {
                    self.solver.add_circle(Circle {
                        point: Particle::new(self.mouse_pos),
                        radius: self.radius,
                    });
                }
            }
            SpawnType::Polygon => {
                self.overlay_circle_polygon(Vector2::zeros());
                if should_spawn {
                    self.solver.add_polygon(Polygon::circle(
                        self.radius,
                        self.mouse_pos,
                        self.point_count,
                        false,
                        self.stiffness,
                        self.permanence_threshold,
                    ));
                }
            }
            SpawnType::PressurePolygon => {
                self.overlay_circle_polygon(Vector2::zeros());
                if should_spawn {
                    println!("pressure polygon: {}", self.pressure);
                    self.solver.add_polygon(Polygon::pressure_circle(
                        self.radius,
                        self.mouse_pos,
                        self.point_count,
                        false,
                        self.stiffness,
                        self.pressure,
                    ));
                }
            }
            SpawnType::Static => {
                self.overlay_line((
                    Vector2::new(0.0, 0.0),
                    Vector2::new(self.radius, self.radius),
                ));
                if should_spawn {
                    self.solver.add_static_line((
                        self.mouse_pos,
                        self.mouse_pos + Vector2::new(self.radius, self.radius),
                    ));
                }
            }
        }
    }

    fn input_grid(&mut self) {
        let should_spawn = is_mouse_button_pressed(MouseButton::Left);
        match self.spawn_type {
            SpawnType::Particle => {
                self.overlay_particle_grid();
                if should_spawn {
                    spawn_particle_array(
                        &mut self.solver,
                        self.mouse_pos,
                        Vector2::new(self.point_count as u32, self.point_count as u32),
                        self.radius,
                        self.stiffness,
                        self.permanence_threshold,
                    );
                }
            }
            SpawnType::Circle => {
                self.overlay_circle_grid();
                if should_spawn {
                    spawn_circle_array(
                        &mut self.solver,
                        self.mouse_pos,
                        Vector2::new(self.point_count as u32, self.point_count as u32),
                        self.radius * 2.0,
                        self.radius,
                    );
                }
            }
            SpawnType::Polygon => {
                for x in 0..10 {
                    for y in 0..10 {
                        self.overlay_circle_polygon(Vector2::new(
                            x as f32 * self.radius * 2.2,
                            y as f32 * self.radius * 2.2,
                        ));
                    }
                }
                if should_spawn {
                    for x in 0..10 {
                        for y in 0..10 {
                            let offset = Vector2::new(
                                x as f32 * self.radius * 2.2,
                                y as f32 * self.radius * 2.2,
                            );
                            self.solver.add_polygon(Polygon::circle(
                                self.radius,
                                self.mouse_pos + offset,
                                self.point_count,
                                false,
                                self.stiffness,
                                self.permanence_threshold,
                            ));
                        }
                    }
                }
            }
            SpawnType::PressurePolygon => {
                for x in 0..10 {
                    for y in 0..10 {
                        self.overlay_circle_polygon(Vector2::new(
                            x as f32 * self.radius * 2.2,
                            y as f32 * self.radius * 2.2,
                        ));
                    }
                }
                if should_spawn {
                    for x in 0..10 {
                        for y in 0..10 {
                            let offset = Vector2::new(
                                x as f32 * self.radius * 2.2,
                                y as f32 * self.radius * 2.2,
                            );
                            self.solver.add_polygon(Polygon::pressure_circle(
                                self.radius,
                                self.mouse_pos + offset,
                                self.point_count,
                                false,
                                self.stiffness,
                                self.pressure,
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn input_last(&mut self) {
        let should_spawn = is_mouse_button_pressed(MouseButton::Left);
        match self.spawn_type {
            SpawnType::Particle => {
                self.overlay_last_particle();
                if should_spawn {
                    self.solver.add_particle(self.mouse_pos);
                    let length = self.solver.get_particle_len();
                    if length < 2 {
                        return;
                    }
                    if let Some(particle) = self.solver.get_particle(length - 2) {
                        // self.solver.add_particle_spring(Spring {
                        //     particle_a: length - 2,
                        //     particle_b: length - 1,
                        //     rest_length: (self.mouse_pos - particle.pos).magnitude(),
                        //     stiffness: 1.0,
                        // });
                        self.solver.add_particle_link(ParticleLink {
                            link: Link {
                                particle_a: length - 2,
                                particle_b: length - 1,
                                target_distance: (self.mouse_pos - particle.pos).magnitude(),
                            },
                        });
                    }
                }
            }
            SpawnType::Circle => {
                self.overlay_last_circle();
                if should_spawn {
                    self.solver.add_circle(Circle {
                        point: Particle::new(self.mouse_pos),
                        radius: self.radius,
                    });
                    let length = self.solver.get_circles_len();
                    if length < 2 {
                        return;
                    }
                    if let Some(particle) = self.solver.get_circle(length - 2) {
                        self.solver.add_circle_link(CircleLink {
                            link: Link {
                                particle_a: length - 2,
                                particle_b: length - 1,
                                target_distance: (self.mouse_pos - particle.point.pos).magnitude(),
                            },
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn input_spam(&mut self) {
        let should_spawn = is_mouse_button_down(MouseButton::Left);
        match self.spawn_type {
            SpawnType::Particle => {
                self.overlay_particle();
                if should_spawn {
                    self.solver.add_particle(self.mouse_pos);
                }
            }
            SpawnType::Circle => {
                self.overlay_circle();
                if should_spawn {
                    self.solver.add_circle(Circle {
                        point: Particle::new(self.mouse_pos),
                        radius: self.radius,
                    });
                }
            }
            _ => {}
        }
    }

    fn overlay_clear(&mut self) {
        self.points_vec.clear();
        self.links_vec.clear();
        self.circles_vec.clear();
    }

    fn overlay_particle(&mut self) {
        self.points_vec.push(Vector2::new(0.0, 0.0));
    }

    fn overlay_circle(&mut self) {
        self.circles_vec.push(Vector2::new(0.0, 0.0));
    }

    fn overlay_line(&mut self, line: (Vector2<f32>, Vector2<f32>)) {
        self.points_vec.push(line.0);
        self.points_vec.push(line.1);
        self.links_vec.push(Vector2::new(
            self.points_vec.len() - 2,
            self.points_vec.len() - 1,
        ));
    }

    fn overlay_last_particle(&mut self) {
        self.points_vec.push(Vector2::new(0.0, 0.0));
        let length = self.solver.get_particle_len();
        if length < 1 {
            return;
        }
        if let Some(particle) = self.solver.get_particle(length - 1) {
            self.points_vec.push(particle.pos - self.mouse_pos);
            self.links_vec.push(Vector2::new(0, 1));
        }
    }

    fn overlay_last_circle(&mut self) {
        self.circles_vec.push(Vector2::new(0.0, 0.0));
        let length = self.solver.get_circles_len();
        if length < 1 {
            return;
        }
        if let Some(particle) = self.solver.get_circle(length - 1) {
            self.circles_vec.push(particle.point.pos - self.mouse_pos);
            self.links_vec.push(Vector2::new(0, 1));
        }
    }

    fn overlay_circle_polygon(&mut self, offset: Vector2<f32>) {
        let count = self.point_count;
        let dist = self.radius;
        let start = self.points_vec.len();
        for i in 0..count {
            let angle = i as f32 / count as f32 * std::f32::consts::PI * 2.0;
            self.points_vec
                .push(offset + Vector2::new(angle.cos() * dist, angle.sin() * dist));
            let length = self.points_vec.len();
            if i > 0 {
                self.links_vec.push(Vector2::new(length - 2, length - 1));
            }
            if i == count - 1 {
                self.links_vec.push(Vector2::new(length - 1, start));
            }
        }
    }

    fn overlay_particle_grid(&mut self) {
        let count = self.point_count;
        let dist = self.radius;
        for y in 0..count {
            for x in 0..count {
                self.points_vec
                    .push(Vector2::new(x as f32 * dist, y as f32 * dist));
                let length = self.points_vec.len();
                self.overlay_grid_link(x, y, count, length);
            }
        }
    }

    fn overlay_circle_grid(&mut self) {
        let count = self.point_count;
        let dist = self.radius * 2.0;
        for y in 0..count {
            for x in 0..count {
                self.circles_vec
                    .push(Vector2::new(x as f32 * dist, y as f32 * dist));
                let length = self.circles_vec.len();
                self.overlay_grid_link(x, y, count, length);
            }
        }
    }

    fn overlay_grid_link(&mut self, x: usize, y: usize, count: usize, length: usize) {
        if x > 0 {
            self.links_vec.push(Vector2::new(length - 2, length - 1));
        }
        if y > 0 {
            self.links_vec
                .push(Vector2::new(length - count - 1, length - 1));

            if x < count - 1 {
                self.links_vec
                    .push(Vector2::new(length - count, length - 1));
            }
        }
        if x > 0 && y > 0 {
            self.links_vec
                .push(Vector2::new(length - count - 2, length - 1));
        }
    }

    fn draw(&mut self) {
        // Circles
        let circles = self.solver.get_circles();
        // Draw circles
        for circle in circles.iter() {
            draw_circle(circle.point.pos.x, circle.point.pos.y, circle.radius, BLUE);
        }
        // Draw circle links
        let circles = self.solver.get_circles();
        for link in self.solver.get_circle_links().iter() {
            let particle_a = circles[link.link.particle_a].point.pos;
            let particle_b = circles[link.link.particle_b].point.pos;
            draw_line(
                particle_a.x,
                particle_a.y,
                particle_b.x,
                particle_b.y,
                1.0,
                GREEN,
            );
        }

        // Particles
        let particles = self.solver.get_particles();
        // Draw particles
        for particle in particles.iter() {
            draw_circle(particle.pos.x, particle.pos.y, 3.0, GREEN);
        }
        // Draw particle links
        for link in self.solver.get_particle_links().iter() {
            let particle_a = particles[link.link.particle_a].pos;
            let particle_b = particles[link.link.particle_b].pos;
            draw_line(
                particle_a.x,
                particle_a.y,
                particle_b.x,
                particle_b.y,
                1.0,
                WHITE,
            );
        }
        // Draw spring links
        for link in self.solver.get_particle_springs().iter() {
            let particle_a = particles[link.particle_a].pos;
            let particle_b = particles[link.particle_b].pos;
            draw_line(
                particle_a.x,
                particle_a.y,
                particle_b.x,
                particle_b.y,
                1.0,
                RED,
            );
        }

        // Draw polygons
        for (p, polygon) in self.solver.get_polygons().iter().enumerate() {
            for i in 0..polygon.particles.len() {
                let point_a = polygon.particles[i];
                let point_b = polygon.particles[(i + 1) % polygon.particles.len()];
                draw_triangle(
                    Vec2::new(point_a.pos.x, point_a.pos.y),
                    Vec2::new(point_b.pos.x, point_b.pos.y),
                    Vec2::new(polygon.center.x, polygon.center.y),
                    GRAY,
                );
                draw_line(
                    point_a.pos.x,
                    point_a.pos.y,
                    point_b.pos.x,
                    point_b.pos.y,
                    3.0,
                    BLACK,
                );
                //draw_text(&format!("{}", i), point_a.pos.x, point_a.pos.y, 20.0, BLACK);
            }

            //draw_text(&format!("{}", p), polygon.center.x, polygon.center.y, 40.0, BLACK);
            // for spring in polygon.particle_springs.iter() {
            //     let point_a = polygon.particles[spring.particle_a];
            //     let point_b = polygon.particles[spring.particle_b];
            //     draw_line(
            //         point_a.pos.x,
            //         point_a.pos.y,
            //         point_b.pos.x,
            //         point_b.pos.y,
            //         1.0,
            //         WHITE,
            //     );
            // }

            if self.draw_aabb {
                //Draw bounding box
                draw_rectangle(
                    polygon.bounds.pos.x,
                    polygon.bounds.pos.y,
                    polygon.bounds.size.x,
                    polygon.bounds.size.y,
                    Color::new(1.0, 0.0, 0.0, 0.5),
                )
            }
        }

        // Draw static lines
        for line in self.solver.get_static_lines().iter() {
            draw_line(line.0.x, line.0.y, line.1.x, line.1.y, 3.0, BLUE);
        }

        let overlay_color = Color::new(1.0, 0.0, 0.0, 0.5);
        // Draw overlay points
        for point in self.points_vec.iter() {
            let point = self.mouse_pos + point;
            draw_circle(point.x, point.y, 3.0, overlay_color);
        }
        // Draw overlay circles
        for circle in self.circles_vec.iter() {
            let circle = self.mouse_pos + circle;
            draw_circle(circle.x, circle.y, self.radius, overlay_color);
        }
        // Draw overlay links
        if self.points_vec.is_empty() {
            for link in self.links_vec.iter() {
                let particle_a = self.mouse_pos + self.circles_vec[link.x];
                let particle_b = self.mouse_pos + self.circles_vec[link.y];
                draw_line(
                    particle_a.x,
                    particle_a.y,
                    particle_b.x,
                    particle_b.y,
                    3.0,
                    overlay_color,
                );
            }
        } else {
            for link in self.links_vec.iter() {
                let particle_a = self.mouse_pos + self.points_vec[link.x];
                let particle_b = self.mouse_pos + self.points_vec[link.y];
                draw_line(
                    particle_a.x,
                    particle_a.y,
                    particle_b.x,
                    particle_b.y,
                    3.0,
                    overlay_color,
                );
            }
        }

        if self.pause && !self.collisions.is_empty() && self.pause_on_collision {
            draw_text(
                format!(
                    "Collision: {}/{}",
                    self.collision_index + 1,
                    self.collisions.len()
                )
                .as_str(),
                30.0,
                80.0,
                50.0,
                RED,
            );
            let collision = self.collisions.get(self.collision_index);
            if let Some(collision) = collision {
                self.collision_phase
                    .draw(collision, self.solver.get_polygons());
            } else {
                self.collisions.clear();
                self.collision_index = 0;
                self.pause = false;
            }
        }
    }

    fn draw_ui(&mut self) {
        ui(|egui_ctx| {
            let hovered = egui::Window::new("Information")
                .show(egui_ctx, |ui| {
                    ui.label(format!("FPS: {}", get_fps().to_string().as_str()));
                    ui.label(format!("ms: {}", self.dt));
                    ui.label(format!("Mouse: {} {}", self.mouse_pos.x, self.mouse_pos.y));
                    ui.label(format!("Radius: {}", self.radius));
                    ui.label(format!("Particles: {}", self.solver.get_particle_len()));
                    ui.label(format!("Circles: {}", self.solver.get_circles_len()));
                    ui.collapsing(
                        format!("Polygons: {}", self.solver.get_polygons_len()),
                        |ui| {
                            for (i, polygon) in self.solver.get_polygons().iter().enumerate() {
                                ui.collapsing(
                                    format!(
                                        "Polygo ID: {}, points: {}",
                                        i,
                                        polygon.particles.len()
                                    ),
                                    |ui| {
                                        for point in polygon.particles.iter() {
                                            ui.label(format!(
                                                " Point: {}, {}",
                                                point.pos.x, point.pos.y
                                            ));
                                        }
                                    },
                                );
                            }
                        },
                    );

                    ui.label(format!(
                        "Polygon intersections: {}",
                        self.solver
                            .get_polygons()
                            .iter()
                            .map(|p| p.collisions.len())
                            .sum::<usize>()
                    ));
                    ui.label(format!("Point count: {}", self.point_count));
                    ui.add(egui::Slider::new(&mut self.point_count, 3..=100).text("Point count"));
                    ui.add(egui::Slider::new(&mut self.stiffness, 0.0..=1000.0).text("Stiffness"));
                    ui.add(
                        egui::Slider::new(&mut self.pressure, 0.0..=100000000.0).text("Pressure"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.permanence_threshold, 0.0..=-1.0)
                            .text("Permanence Threshold"),
                    );

                    ui.label(format!("Spawn mode: {}", self.spawn_mode.name()));
                    if ui.button("Change mode").clicked() {
                        self.spawn_mode.increase();
                    }
                    ui.label(format!("Test case: {}", self.test_case.name()));
                    if ui.button("Change case").clicked() {
                        self.test_case.increase();
                        self.solver = Solver::new();
                        self.solver.bounds.size = Vector2::new(screen_width(), screen_height());
                        self.solver.gravity = self.gravity;
                        match self.test_case {
                            TestCase::Playground => {}
                            TestCase::Triangle1 => {
                                self.solver.add_polygon(Polygon::circle(
                                    100.0,
                                    Vector2::new(300.0, 300.0),
                                    3,
                                    false,
                                    100.0,
                                    self.permanence_threshold,
                                ));
                                self.solver.add_polygon(Polygon::circle(
                                    100.0,
                                    Vector2::new(300.0, 600.0),
                                    3,
                                    false,
                                    100.0,
                                    self.permanence_threshold,
                                ));
                            }
                            TestCase::Triangle2 => {
                                self.solver.add_polygon(Polygon::circle(
                                    100.0,
                                    Vector2::new(300.0, 300.0),
                                    3,
                                    true,
                                    5.0,
                                    self.permanence_threshold,
                                ));
                                self.solver.add_polygon(Polygon::circle(
                                    20.0,
                                    Vector2::new(600.0, 300.0),
                                    3,
                                    true,
                                    25.0,
                                    self.permanence_threshold,
                                ));
                            }
                            _ => {}
                        }
                    }
                    if ui
                        .button(match self.draw_aabb {
                            true => "Don't draw bounding boxes",
                            false => "Draw bounding boxes",
                        })
                        .clicked()
                    {
                        self.draw_aabb = !self.draw_aabb;
                    }
                    let pause_on_collision_text = match self.pause_on_collision {
                        true => "Don't pause on collision",
                        false => "Pause on collision",
                    };
                    if ui.button(pause_on_collision_text).clicked() {
                        self.pause_on_collision = !self.pause_on_collision;
                    }
                    ui.label(format!("Spawn type: {}", self.spawn_type.name()));
                    if ui.button("Change type").clicked() {
                        self.spawn_type.increase();
                    }
                    if ui
                        .button(match self.pause {
                            true => "Resume",
                            false => "Pause",
                        })
                        .clicked()
                    {
                        self.pause = !self.pause;
                    }
                    if self.pause && ui.button("Step").clicked() {
                        self.step = true;
                    }
                    if ui.button("Reset").clicked() {
                        self.solver = Solver::new();
                        self.solver.bounds.size = Vector2::new(screen_width(), screen_height());
                        self.solver.gravity = self.gravity;
                    }
                })
                .unwrap()
                .response
                .rect
                .contains(Pos2 {
                    x: self.mouse_pos.x,
                    y: self.mouse_pos.y,
                });
            self.ui_hovered = hovered;
        });

        egui_macroquad::draw();
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(1920.0, 1080.0);
    rand::srand(get_time() as u64);

    // Refresh window
    clear_background(RED);
    next_frame().await;

    let mut testbed = Testbed::new();
    loop {
        // if get_time() - last_update < 0.1 {
        //     continue;
        // }
        // last_update = get_time();
        clear_background(WHITE);

        testbed.update();

        testbed.draw();
        testbed.draw_ui();

        next_frame().await
    }
}
