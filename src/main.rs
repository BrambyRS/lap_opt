use clap::{Parser, ValueEnum};

mod collocation;
mod ipopt;
mod logger;
mod model;
mod track;

// Available model types
#[derive(Debug, Clone, ValueEnum)]
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
}
