use clap::{Parser, ValueEnum};
use simulation_toolbox::erk::ExplicitRK;
use std::fs::File;
use std::io::Write;

mod collocation;
mod ipopt;
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
}

fn main() {
    let args = Args::parse();

    println!("Model: {:?}", args.model);
    println!("Track: {:?}", args.track);
}
