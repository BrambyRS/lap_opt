mod ipopt;
mod problem_data;

use crate::model;
use simulation_toolbox;
use std::os::raw::{c_int, c_void};

// IPOPT callback helper functions
fn get_node_state(x: &[f64], nx: usize, nu: usize, i: usize) -> Vec<f64> {
    return x[i * (nx + nu)..i * (nx + nu) + nx].to_vec();
}

fn get_node_control(x: &[f64], nx: usize, nu: usize, i: usize) -> Vec<f64> {
    return x[i * (nx + nu) + nx..(i + 1) * (nx + nu)].to_vec();
}

// TODO: We can reduce model evaluations by chaching the model dynamics and jacobian
// at each node and then reusing them the calculate the objective, gradients, and hessian.
// Main IPOPT callback implementations for the problem
// -----------------------------------------------------

// Objective evaluation callback
// Objective is to minimise lap time, which is the integral of dt/ds along the trajectory.
// This is implemented in the eval_f callback by evaluating the model dynamics at each collocation point to get the velocity,
// and then using the quadrature weights to compute the integral of dt/ds = 1/v.
extern "C" fn eval_f<T: simulation_toolbox::Model>(
    n_x_raw: ipopt::Index,
    x_raw: *const ipopt::Number,
    _new_x: c_int,
    obj_value: *mut ipopt::Number,
    user_data: *mut c_void,
) -> c_int {
    let problem_data: &problem_data::ProblemData<T> =
        unsafe { &*(user_data as *const problem_data::ProblemData<T>) };
    let x_slice: &[f64] = unsafe { std::slice::from_raw_parts(x_raw, n_x_raw as usize) };

    // The objective is to minimize the average velcoity over the trajectory
    // Evaluate the model dynamics
    let model_nx: usize = problem_data.model.n_x();
    let model_nu: usize = problem_data.model.n_u();

    let mut dt_ds: Vec<f64> = Vec::with_capacity(problem_data.n_mesh);

    for i in 0..problem_data.n_mesh {
        let x_i: Vec<f64> = get_node_state(x_slice, model_nx, model_nu, i);

        let track_tangent: (f64, f64) = problem_data.track_mesh[i].tangent();
        let dt_ds_i: f64 = model::get_scaling(x_i, vec![track_tangent.0, track_tangent.1]);

        dt_ds.push(dt_ds_i); // dt/ds is 1/v, so minimising the integral of dt/ds is minimising lap time
    }

    unsafe {
        *obj_value = problem_data.integrate_cost(&dt_ds);
    }

    return 1; // true
}

#[cfg(test)]
mod tests {
    use crate::collocation;
    use crate::track;
    use problem_data::ProblemData;
    use simulation_toolbox::Model;

    use super::*;

    struct PointMass2D {
        mass: f64,
    }

    impl Model for PointMass2D {
        fn name(&self) -> &str {
            return "PointMass2D";
        }

        fn n_x(&self) -> usize {
            return 4;
        }

        fn n_u(&self) -> usize {
            return 2;
        }

        fn fun(&self, x: &Vec<f64>, u: &Vec<f64>, _t: f64) -> Vec<f64> {
            return vec![x[2], x[3], u[0] / self.mass, u[1] / self.mass];
        }

        fn jac(&self, _x: &Vec<f64>, _u: &Vec<f64>, _t: f64) -> Vec<f64> {
            //          x0   x1   x2   x3   u0   u1   t
            return vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.0, 0.0, 1.0/self.mass, 0.0, 0.0,
                        0.0, 0.0, 0.0, 0.0, 0.0, 1.0/self.mass, 0.0,
                        ];
        }
    }

    impl PointMass2D {
        pub fn new(mass: f64) -> Self {
            return PointMass2D { mass };
        }

        fn hess(&self, _x: &Vec<f64>, _u: &Vec<f64>, _t: f64) -> Vec<f64> {
            return vec![0.0; 49]; // 7x7 zero matrix (4 states + 2 controls + 1 time)
        }
    }

    #[test]
    fn test_get_state_and_control() {
        let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let nx: usize = 2;
        let nu: usize = 1;

        let state_0: Vec<f64> = get_node_state(&x, nx, nu, 0);
        let control_0: Vec<f64> = get_node_control(&x, nx, nu, 0);
        let state_1: Vec<f64> = get_node_state(&x, nx, nu, 1);
        let control_1: Vec<f64> = get_node_control(&x, nx, nu, 1);

        assert_eq!(state_0, vec![1.0, 2.0]);
        assert_eq!(control_0, vec![3.0]);
        assert_eq!(state_1, vec![4.0, 5.0]);
        assert_eq!(control_1, vec![6.0]);
    }

    #[test]
    fn test_eval_f_straight() {
        let model = PointMass2D::new(1.0);
        let track = track::Track::straight(5.0, 1.0);
        let quadrature = match collocation::FLGR::new(2) {
            Some(q) => q,
            None => panic!("Failed to create FLGR collocation of order 2"),
        };

        let n_segments: usize = 2;
        let problem_data: ProblemData<PointMass2D> =
            ProblemData::new(model, track, quadrature, n_segments);

        // Just assume the point has a constant velocity of 1 m/s in x (along the track)
        // The positions don't matter, as only the velocity affects the objective
        let x: Vec<f64> = vec![
        //  x0   x1   x2   x3   u0   u1
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Node 0 (initial point)
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Node 1 (Segment 1, Collocation point 1)
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Node 2 (Segment 2, Collocation point 2)
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Node 3 (Segment 2, Collocation point 1)
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, // Node 4 (Segment 2, Collocation point 2)
        ];

        let mut obj_value: f64 = 0.0;
        let x_raw: *const f64 = x.as_ptr();
        let user_data: *const c_void = &problem_data as *const _ as *const c_void;
        let success: c_int = eval_f::<PointMass2D>(
            x.len() as ipopt::Index,
            x_raw,
            0, // new_x is false
            &mut obj_value as *mut f64,
            user_data as *mut c_void,
        );

        assert_eq!(success, 1);
        // With a constant velocity of 1 m/s along the track, the lap time should be 5 s on a 5 m track
        assert!(
            (obj_value - 5.0).abs() < 1e-4,
            "Objective value is {}, expected 5.0",
            obj_value
        );

        // Test with y-component of velocity, which should not affect the objective as the track is straight along x
        let x_with_y_vel: Vec<f64> = vec![
        //  x0   x1   x2   x3   u0   u
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Node 0 (initial point)
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Node 1 (Segment 1, Collocation point 1)
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Node 2 (Segment 2, Collocation point 2)
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Node 3 (Segment 2, Collocation point 1)
            0.0, 0.0, 1.0, 1.0, 0.0, 0.0, // Node 4 (Segment 2, Collocation point 2)
        ];

        let mut obj_value_with_y_vel: f64 = 0.0;
        let x_with_y_vel_raw: *const f64 = x_with_y_vel.as_ptr();
        let success_with_y_vel: c_int = eval_f::<PointMass2D>(
            x_with_y_vel.len() as ipopt::Index,
            x_with_y_vel_raw,
            0, // new_x is false
            &mut obj_value_with_y_vel as *mut f64,
            user_data as *mut c_void,
        );
        assert_eq!(success_with_y_vel, 1);
        // The objective value should be the same as before, since the velocity along the track is
        // still 1 m/s, so the lap time should still be 5 s
        assert!(
            (obj_value_with_y_vel - 5.0).abs() < 1e-4,
            "Objective value with y velocity is {}, expected 5.0",
            obj_value_with_y_vel
        );
    }
}
