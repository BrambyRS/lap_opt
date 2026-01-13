#![allow(dead_code)]

use crate::{collocation, track};
use simulation_toolbox::Model;

struct ProblemData<T: Model> {
    model: T,
    track: track::Track,
    quadrature: collocation::FLGR,

    is_initialised: bool,
    is_solved: bool,

    initial_solution: Option<Vec<f64>>,
    solution: Option<Vec<f64>>,
}

impl<T: Model> ProblemData<T> {
    pub fn new(model: T, track: track::Track, quadrature: collocation::FLGR) -> Self {
        return ProblemData {
            model,
            track,
            quadrature,
            is_initialised: false,
            is_solved: false,
            initial_solution: None,
            solution: None,
        };
    }

    pub fn initialise(&mut self, n_segments: usize) -> Result<(), &str> {
        if self.is_initialised {
            return Err("ProblemData is already initialised.");
        }

        self.is_initialised = true;
        return Ok(());
    }

    pub fn solve(&mut self) -> Result<(), &str> {
        if !self.is_initialised {
            return Err("ProblemData is not initialised.");
        }
        // Solve the problem here
        self.is_solved = true;
        return Ok(());
    }

    // Getters
    pub fn is_initialised(&self) -> bool {
        return self.is_initialised;
    }

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
        if !self.is_initialised {
            return None;
        }
        return None; // TODO: Implement interpolation logic
    }
}
