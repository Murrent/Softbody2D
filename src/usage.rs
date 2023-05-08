use bendy2d::polygon::Polygon;
use bendy2d::solver::Solver;
use nalgebra::Vector2;

fn main() {
    let mut solver = Solver::new();

    solver.add_polygon(Polygon::circle(
        3.0,
        Vector2::new(0.0, 0.0),
        4,
        false,
        100.0,
    ));

    loop {
        // Add forces to polygons here

        solver.update(0.05);

        let polygons = solver.get_polygons();
        // Draw the polygons here
    }
}
