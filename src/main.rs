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
}

impl SpawnType {
    fn name(&self) -> &str {
        match *self {
            SpawnType::Particle => "Particle",
            SpawnType::Circle => "Circle",
            SpawnType::Polygon => "Polygon",
        }
    }

    fn increase(&mut self) {
        *self = match *self {
            SpawnType::Particle => SpawnType::Circle,
            SpawnType::Circle => SpawnType::Polygon,
            SpawnType::Polygon => SpawnType::Particle,
        }
    }
}

enum TestCase {
    Triangle1,
    Triangle2,
    Triangle3,
    Circle1,
    Circle2,
}

impl TestCase {
    fn name(&self) -> &str {
        match *self {
            TestCase::Triangle1 => "Triangle1",
            TestCase::Triangle2 => "Triangle2",
            TestCase::Triangle3 => "Triangle3",
            TestCase::Circle1 => "Circle1",
            TestCase::Circle2 => "Circle2",
        }
    }
    fn increase(&mut self) {
        *self = match *self {
            TestCase::Triangle1 => TestCase::Triangle2,
            TestCase::Triangle2 => TestCase::Triangle3,
            TestCase::Triangle3 => TestCase::Circle1,
            TestCase::Circle1 => TestCase::Circle2,
            TestCase::Circle2 => TestCase::Triangle1,
        }
    }
}

fn spawn_particle_array(solver: &mut Solver, pos: Vector2<f32>, count: Vector2<u32>, dist: f32) {
    for y in 0..count.y {
        for x in 0..count.x {
            let particle_pos = Vector2::new(pos.x + x as f32 * dist, pos.y + y as f32 * dist);
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

    radius: f32,
    spawn_mode: SpawnMode,
    test_case: TestCase,
    spawn_type: SpawnType,
    point_count: usize,
    // Overlay vectors
    points_vec: Vec<Vector2<f32>>,
    circles_vec: Vec<Vector2<f32>>,
    links_vec: Vec<Vector2<usize>>,

    ui_hovered: bool,
    pause: bool,
    mouse_pos: Vector2<f32>,
    dt: f32,
}

impl Testbed {
    fn new() -> Self {
        let mut solver = Solver::new();
        solver.bounds.size = Vector2::new(screen_width(), screen_height());
        solver.gravity = Vector2::new(0.0, 1000.0);

        let radius = 10.0;
        let spawn_mode = SpawnMode::Single;
        let test_case = TestCase::Triangle1;
        let spawn_type = SpawnType::Particle;
        let point_count = 5;
        let points_vec = Vec::<Vector2<f32>>::new();
        let circles_vec = Vec::<Vector2<f32>>::new();
        let links_vec = Vec::<Vector2<usize>>::new();

        let ui_hovered = false;
        let pause = false;
        let mouse_pos = Vector2::<f32>::new(0.0, 0.0);
        let dt = 0.0;

        Self {
            solver,
            radius,
            spawn_mode,
            test_case,
            spawn_type,
            point_count,
            points_vec,
            circles_vec,
            links_vec,
            ui_hovered,
            pause,
            mouse_pos,
            dt,
        }
    }

    fn update(&mut self) {
        self.dt = 0.005; //get_frame_time();
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

            if !self.pause {
                self.solver.update(self.dt);
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
            SpawnType::Polygon => {
                self.overlay_circle_polygon(Vector2::zeros());
                if should_spawn {
                    self.solver.add_polygon(Polygon::circle(
                        self.radius,
                        self.mouse_pos,
                        self.point_count,
                        false,
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
                        Vector2::new(5, 5),
                        self.radius,
                    );
                }
            }
            SpawnType::Circle => {
                self.overlay_circle_grid();
                if should_spawn {
                    spawn_circle_array(
                        &mut self.solver,
                        self.mouse_pos,
                        Vector2::new(5, 5),
                        self.radius * 2.0,
                        self.radius,
                    );
                }
            }
            SpawnType::Polygon => {
                for x in 0..5 {
                    for y in 0..5 {
                        self.overlay_circle_polygon(Vector2::new(
                            x as f32 * self.radius * 2.2,
                            y as f32 * self.radius * 2.2,
                        ));
                    }
                }
                if should_spawn {
                    for x in 0..5 {
                        for y in 0..5 {
                            let offset = Vector2::new(
                                x as f32 * self.radius * 2.2,
                                y as f32 * self.radius * 2.2,
                            );
                            self.solver.add_polygon(Polygon::circle(
                                self.radius,
                                self.mouse_pos + offset,
                                self.point_count,
                                false,
                            ));
                        }
                    }
                }
            }
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
            SpawnType::Polygon => {}
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
            SpawnType::Polygon => {}
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

    fn draw(&self) {
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
                WHITE,
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

        // Draw polygons
        for polygon in self.solver.get_polygons().iter() {
            for i in 0..polygon.particles.len() {
                let point_a = polygon.particles[i];
                let point_b = polygon.particles[(i + 1) % polygon.particles.len()];
                draw_line(
                    point_a.pos.x,
                    point_a.pos.y,
                    point_b.pos.x,
                    point_b.pos.y,
                    3.0,
                    ORANGE,
                );
            }
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
                    ui.label(format!("Polygons: {}", self.solver.get_polygons_len()));
                    ui.label(format!("Point count: {}", self.point_count));
                    ui.add(egui::Slider::new(&mut self.point_count, 3..=100).text("Point count"));

                    ui.label(format!("Spawn mode: {}", self.spawn_mode.name()));
                    if ui.button("Change mode").clicked() {
                        self.spawn_mode.increase();
                    }
                    ui.label(format!("Test case: {}", self.test_case.name()));
                    if ui.button("Change case").clicked() {
                        self.test_case.increase();
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
                    if ui.button("Reset").clicked() {
                        self.solver = Solver::new();
                        self.solver.bounds.size = Vector2::new(screen_width(), screen_height());
                        self.solver.gravity = Vector2::new(0.0, 1000.0);
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
    request_new_screen_size(1280.0, 720.0);
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
        clear_background(BLACK);

        testbed.update();

        testbed.draw();
        testbed.draw_ui();

        next_frame().await
    }
}
