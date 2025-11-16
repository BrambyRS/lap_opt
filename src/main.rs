use simulation_toolbox::erk::ExplicitRK;
use std::fs::File;
use std::io::Write;

mod model;
mod track;

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

    let test_track: track::Track =
        track::Track::read_from_file("/Users/rsingh/Repos/lap_opt/tracks/gbg_city_arena.trk");
    println!("{}", test_track);

    let n_frames: usize = 1000;
    let ds: f64 = test_track.length() / (n_frames as f64);
    let mut s_lap_q: Vec<f64> = Vec::with_capacity(n_frames);
    for i in 0..n_frames {
        s_lap_q.push(i as f64 * ds);
    }

    let track_frames: Box<Vec<track::TrackFrame>> = test_track.discretise(s_lap_q);

    let csv_path: &str = "track_points.csv";
    let mut track_file: File = File::create(csv_path).unwrap();
    writeln!(track_file, "xc,yc,xl,yl,xr,yr").unwrap();
    for frame in track_frames.iter() {
        let (xc, yc) = frame.position();
        let (nx, ny) = frame.lateral();
        let width: f64 = frame.width();

        let xl: f64 = xc + (width / 2.0) * nx;
        let yl: f64 = yc + (width / 2.0) * ny;
        let xr: f64 = xc - (width / 2.0) * nx;
        let yr: f64 = yc - (width / 2.0) * ny;

        writeln!(track_file, "{xc},{yc},{xl},{yl},{xr},{yr}").unwrap();
    }
}
