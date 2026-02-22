#![allow(dead_code)]

use crate::{collocation, track};
use simulation_toolbox::Model;

/* ProblemData is specifically for the lap time optimisation problem.
It encapsulates the model, track, quadrature scheme, and problem variables.
The problem structure defintions are written with the lap time optimistaion problem in mind,
although the exact objective and contraint functions are implemented in the problem module
above this. */
pub struct ProblemData<T: Model> {
    // Core problem properties/components
    pub model: T,
    track: track::Track,
    quadrature: collocation::FLGR,
    is_solved: bool,

    s_mesh: Vec<f64>, // The mesh points along the track, including the initial point
    s_mesh_colloc: Vec<f64>, // The collocation points along the track, including the initial point
    pub track_mesh: Box<Vec<track::TrackFrame>>, // The mesh points along the track, including the initial point
    pub n_mesh: usize, // Number of mesh elements (including initial point)

    initial_solution: Option<Vec<f64>>,
    solution: Option<Vec<f64>>,

    // Problem variables
    x_dec: Vec<f64>, // Decision variables [x0, u0, x1, u1, ..., xn, un]

    // Problem dimensions
    nx_dec: usize, // Number of decision variables
    nnz_jac_g: usize, // Number of non-zero elements in the Jacobian
    nnz_h_lag: usize, // Number of non-zero elements in the Hessian of the Lagrangian
    
    // Problem structure
    jac_g_shape: Vec<(usize, usize)>, // Indices of the non-zero elements in the Jacobian
    h_lag_shape: Vec<(usize, usize)>, // Indices of the non-zero elements in the Hessian of the Lagrangian
}

impl<T: Model> ProblemData<T> {
    pub fn new(model: T, track: track::Track, quadrature: collocation::FLGR, n_segments: usize) -> Self {
        
        // Problem dimensions
        let n_mesh: usize = n_segments * quadrature.n_q + 1; // Total number of mesh points (including initial point)
        let n_states_dec: usize = model.n_x() * n_mesh; // One per collocation node plus one for the initial point
        let n_controls_dec: usize = model.n_u() * n_mesh; // One per collocation node plus one for the initial point
        
        // Get track mesh
        let track_length: f64 = track.length();
        let s_mesh: Vec<f64> = (0..=n_segments).map(|i| i as f64 * track_length / n_segments as f64).collect();
        let mut s_mesh_colloc: Vec<f64> = Vec::with_capacity(n_segments * quadrature.n_q);
        for i in 0..n_segments {
            let a: f64 = s_mesh[i];
            let b: f64 = s_mesh[i+1];
            s_mesh_colloc.extend(quadrature.map_nodes(a, b));
        }
        let track_mesh: Box<Vec<track::TrackFrame>> = track.discretise(&s_mesh_colloc);

        // For now, we're assuming the model jacobian and hessian are dense
        // TODO: Account for the sparsity structure of the model
        let nnz_jac_model: usize = model.n_x() * (model.n_x() + model.n_u() + 1); // +1 for time dependent models
        let nnz_hess_model: usize = (model.n_x() + model.n_u()).pow(2);

        let nnz_jac_g: usize = nnz_jac_model * n_mesh;
        let nnz_h_lag: usize = nnz_hess_model * n_mesh;

        let mut jac_g_shape: Vec<(usize, usize)> = Vec::with_capacity(nnz_jac_g);
        let mut h_lag_shape: Vec<(usize, usize)> = Vec::with_capacity(nnz_h_lag);

        // TODO: Implement proper sparsity structure generation

        return ProblemData {
            model,
            track,
            quadrature,
            s_mesh,
            s_mesh_colloc,
            track_mesh,
            n_mesh,
            is_solved: false,
            initial_solution: None,
            solution: None,
            nx_dec: n_states_dec + n_controls_dec,
            x_dec: vec![0.0; n_states_dec + n_controls_dec],
            nnz_jac_g,
            nnz_h_lag,
            jac_g_shape,
            h_lag_shape,
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

#[cfg(test)]
mod tests {
    use super::*;

    struct PointMass1D {
        mass: f64,
    }
    
    impl Model for PointMass1D {
        fn name(&self) -> &str {
            return "PointMass1D";
        }

        fn n_x(&self) -> usize {
            return 2;
        }

        fn n_u(&self) -> usize {
            return 1;
        }

        fn fun(&self, x: &Vec<f64>, u: &Vec<f64>, _t: f64) -> Vec<f64> {
            return vec![x[1], u[0] / self.mass];
        }

        fn jac(&self, _x: &Vec<f64>, _u: &Vec<f64>, _t: f64) -> Vec<f64> {
            //          x0   x1   u0   t
            return vec![0.0, 1.0, 0.0, 0.0,
                        0.0, 0.0, 1.0/self.mass, 0.0];
        }
    }

    impl PointMass1D {
        pub fn new(mass: f64) -> Self {
            return PointMass1D { mass };
        }

        fn hess(&self, _x: &Vec<f64>, _u: &Vec<f64>, _t: f64) -> Vec<f64> {
            return vec![0.0; 9]; // 3x3 zero matrix
        }
    }

    #[test]
    fn test_problem_structure_1nq() {
        let model: PointMass1D = PointMass1D::new(1.0);
        let track = track::Track::straight(100.0, 1.0);
        let quadrature: collocation::FLGR = match collocation::FLGR::new(1) {
            Some(q) => q,
            None => panic!("Failed to create FLGR collocation of order 1"),
        };

        let n_segments: usize = 5;
        let problem_data: ProblemData<PointMass1D> = ProblemData::new(model, track, quadrature, n_segments);

        // Single quadrature point per segment means as many quadrature points as segments
        // So there should be 2 states + 1 control per segment, plus 2 states + 1 control for the initial point
        // This results in 12 states + 6 controls = 18 decision variables
        assert_eq!(problem_data.nx_dec, 18);

        // With a single quadrature point per element, the s_mesh and s_mesh_colloc should be the same
        // Except for the first element, which is not included in the collocation mesh
        assert_eq!(problem_data.s_mesh.len(), n_segments + 1);
        assert_eq!(problem_data.s_mesh_colloc.len(), n_segments);
        for i in 0..n_segments {
            assert!((problem_data.s_mesh[i+1] - problem_data.s_mesh_colloc[i]).abs() < 1e-6);
        }
    }
}