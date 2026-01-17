#![allow(dead_code)]

use crate::{collocation, track};
use simulation_toolbox::Model;

struct ProblemData<T: Model> {
    // Core problem properties/components
    model: T,
    track: track::Track,
    quadrature: collocation::FLGR,
    is_solved: bool,

    initial_solution: Option<Vec<f64>>,
    solution: Option<Vec<f64>>,
}

impl<T: Model> ProblemData<T> {
    pub fn new(model: T, track: track::Track, quadrature: collocation::FLGR, n_segments: usize) -> Self {
        // TODO: Initialise problem here
        
        return ProblemData {
            model,
            track,
            quadrature,
            is_solved: false,
            initial_solution: None,
            solution: None,
        };
    }

    pub fn solve(&mut self) -> Result<(), &str> {
        // Solve the problem here
        self.is_solved = true;
        return Ok(());
    }

    // Getters
    pub fn is_solved(&self) -> bool {
        return self.is_solved;
    }

    // Solution Accessors
    pub fn get_solution(&self) -> Option<&Vec<f64>> {
        // TODO: Deconstruct solution into states and controls
        return self.solution.as_ref();
    }

    pub fn get_initial_solution(&self) -> Option<&Vec<f64>> {
        // TODO: Deconstruct initial solution into states and controls
        return self.initial_solution.as_ref();
    }

    pub fn interpolate_solution(&self, sq: Vec<f64>) -> Option<Vec<f64>> {
        if !self.is_solved {
            return None;
        }
        return None; // TODO: Implement interpolation logic
    }

    pub fn interpolate_initial_solution(&self, sq: Vec<f64>) -> Option<Vec<f64>> {
        return None; // TODO: Implement interpolation logic
    }
}
