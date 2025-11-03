use simulation_toolbox::erk::ExplicitRK;
use std::fs::File;
use std::io::Write;
mod model;

fn main() {
    let solver: ExplicitRK = ExplicitRK::rk4();
    let point_mass: model::point_mass::PointMass =
        model::point_mass::PointMass::new("PointMass1", 1.0);

    let x0: Vec<f64> = vec![0.0, 0.0, 0.0, 0.0]; // Initial state: [x, y, vx, vy]
    let u: Vec<f64> = vec![1.0, 0.0]; // Control input: [Fx, Fy]
    let t0: f64 = 0.0;
    let tf: f64 = 10.0;
    let dt: f64 = 0.1;
    let mut x: Vec<f64> = x0.clone();
    let mut t: f64 = t0;

    let mut file: File = File::create("sim_out.csv").unwrap();
    writeln!(file, "time,x,y,vx,vy").unwrap();

    while t < tf {
        // Update the state using the solver
        x = solver.step(&point_mass, &x, &u, t, dt);
        t += dt;

        writeln!(file, "{},{},{},{},{}", t, x[0], x[1], x[2], x[3]).unwrap();
    }
}
