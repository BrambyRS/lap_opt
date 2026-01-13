use clap::{Parser, ValueEnum};

mod collocation;
mod logger;
mod model;
mod problem;
mod track;

// Available model types
#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum ModelType {
    PointMass,
}

#[derive(Debug, Clone, ValueEnum)]
enum TrackType {
    DoubleLaneChange,
    Straight,
}

// APEX is a lap time optimisation tool using direct collocation.
//
// APEX solves the optimal control problem of minimising lap time given a vehicle model and track using a direct collocation framework. The resulting nonlinear program is solved using IPOPT with the MA27 solver.
#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    // Model to simulate
    #[arg(short, long)]
    model: ModelType,

    // Path to the track file
    #[arg(short, long)]
    track: TrackType,

    // Verbose output
    #[arg(long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args: Args = Args::parse();
    let logger: logger::Logger = logger::Logger::new(args.verbose);

    logger.log_info(&format!("Model: {:?}", args.model));
    logger.log_info(&format!("Track: {:?}", args.track));

    let track: track::Track = match args.track {
        TrackType::DoubleLaneChange => track::Track::double_lane_change(),
        TrackType::Straight => track::Track::straight(1.0, 2.0),
    };

    if args.model == ModelType::PointMass {
        let model: model::point_mass::PointMass =
            model::point_mass::PointMass::new("Point Mass", 1500.0);
    }

    let n_segments: usize = 1;
    let n_quadrature: usize = 3;
    let nc: usize = n_segments * n_quadrature + 1; // Number of total collocation nodes including first point
    logger.log_info(&format!("Discretising track into {n_segments} segments using {n_quadrature} quadrature points per segment."));
    let flgr: collocation::FLGR = match collocation::FLGR::new(n_quadrature) {
        Some(colloc) => colloc,
        None => {
            logger.log_error(&format!(
                "Failed to create FLGR collocation of degree {n_quadrature}"
            ));
            return;
        }
    };

    let mut s: Vec<f64> = Vec::<f64>::with_capacity(nc);
    let l: f64 = track.length();
    s.push(0.0); // First node at s=0
                 // All other according to collocation
    for i in 0..n_segments {
        let s0: f64 = (i as f64) / (n_segments as f64) * l;
        let s1: f64 = (i as f64 + 1.0) / (n_segments as f64) * l;
        let sq: Vec<f64> = flgr.map_nodes(s0, s1);
        for &node in &sq {
            s.push(node);
        }
    }
    logger.log_debug(&format!("Collocation nodes s: {:?}", s));
    let discrete_track: Box<Vec<track::TrackFrame>> = track.discretise(s);
}
