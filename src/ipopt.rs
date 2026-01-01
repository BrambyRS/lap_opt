#![allow(dead_code)]

use std::os::raw::{c_char, c_double, c_int, c_void};

// IPOPT C API types and constants
type Index = c_int;
type Number = c_double;
type Integer = c_int;

#[repr(C)]
#[derive(Debug, PartialEq)]
enum ApplicationReturnStatus {
    SolveSucceeded = 0,
    SolvedToAcceptableLevel = 1,
    InfeasibleProblemDetected = 2,
    SearchDirectionBecomesTooSmall = 3,
    DivergingIterates = 4,
    UserRequestedStop = 5,
    FeasiblePointFound = 6,
    MaximumIterationsExceeded = -1,
    RestorationFailed = -2,
    ErrorInStepComputation = -3,
    MaximumCpuTimeExceeded = -4,
    NotEnoughDegreesOfFreedom = -10,
    InvalidProblemDefinition = -11,
    InvalidOption = -12,
    InvalidNumberDetected = -13,
    UnrecoverableException = -100,
    NonIpoptExceptionThrown = -101,
    InsufficientMemory = -102,
    InternalError = -199,
}

// Callback function type definitions
// Objective function evaluation
type EvalF = extern "C" fn(
    n: Index,
    x: *const Number,
    new_x: c_int,
    obj_value: *mut Number,
    user_data: *mut c_void,
) -> c_int;

// Objective function gradient evaluation
type EvalGradF = extern "C" fn(
    n: Index,
    x: *const Number,
    new_x: c_int,
    grad_f: *mut Number,
    user_data: *mut c_void,
) -> c_int;

// Constraint function evaluation
type EvalG = extern "C" fn(
    n: Index,
    x: *const Number,
    new_x: c_int,
    m: Index,
    g: *mut Number,
    user_data: *mut c_void,
) -> c_int;

// Constraint Jacobian evaluation
type EvalJacG = extern "C" fn(
    n: Index,
    x: *const Number,
    new_x: c_int,
    m: Index,
    nele_jac: Index,
    i_row: *mut Index,
    j_col: *mut Index,
    values: *mut Number,
    user_data: *mut c_void,
) -> c_int;

// Hessian of the Lagrangian evaluation
type EvalH = extern "C" fn(
    n: Index,
    x: *const Number,
    new_x: c_int,
    obj_factor: Number,
    m: Index,
    lambda: *const Number,
    new_lambda: c_int,
    nele_hess: Index,
    i_row: *mut Index,
    j_col: *mut Index,
    values: *mut Number,
    user_data: *mut c_void,
) -> c_int;

// IPOPT C API functions
#[link(name = "ipopt")]
extern "C" {
    fn CreateIpoptProblem(
        n: Index,
        x_L: *const Number,
        x_U: *const Number,
        m: Index,
        g_L: *const Number,
        g_U: *const Number,
        nele_jac: Index,
        nele_hess: Index,
        index_style: Index,
        eval_f: EvalF,
        eval_g: EvalG,
        eval_grad_f: EvalGradF,
        eval_jac_g: EvalJacG,
        eval_h: EvalH,
    ) -> *mut c_void;

    fn FreeIpoptProblem(ipopt_problem: *mut c_void);

    fn AddIpoptStrOption(
        ipopt_problem: *mut c_void,
        keyword: *const c_char,
        val: *const c_char,
    ) -> c_int;

    fn AddIpoptNumOption(ipopt_problem: *mut c_void, keyword: *const c_char, val: Number) -> c_int;
    fn AddIpoptIntOption(ipopt_problem: *mut c_void, keyword: *const c_char, val: Integer)
        -> c_int;

    fn IpoptSolve(
        ipopt_problem: *mut c_void,
        x: *mut Number,
        g: *mut Number,
        obj_val: *mut Number,
        mult_g: *mut Number,
        mult_x_L: *mut Number,
        mult_x_U: *mut Number,
        user_data: *mut c_void,
    ) -> ApplicationReturnStatus;
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::ffi::CString;
    use std::ptr;

    // Callback implementations for the test problem
    // Objective: f(x) = (x1 - 1)^2 + (x2 - 2.5)^2
    extern "C" fn eval_f(
        n: Index,
        x: *const Number,
        _new_x: c_int,
        obj_value: *mut Number,
        _user_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let x_slice = std::slice::from_raw_parts(x, n as usize);
            *obj_value = (x_slice[0] - 1.0).powi(2) + (x_slice[1] - 2.5).powi(2);
        }
        1 // true
    }

    // Gradient: grad_f = [2*(x1 - 1), 2*(x2 - 2.5)]
    extern "C" fn eval_grad_f(
        n: Index,
        x: *const Number,
        _new_x: c_int,
        grad_f: *mut Number,
        _user_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let x_slice = std::slice::from_raw_parts(x, n as usize);
            let grad_slice = std::slice::from_raw_parts_mut(grad_f, n as usize);
            grad_slice[0] = 2.0 * (x_slice[0] - 1.0);
            grad_slice[1] = 2.0 * (x_slice[1] - 2.5);
        }
        1 // true
    }

    // No constraints
    extern "C" fn eval_g(
        _n: Index,
        _x: *const Number,
        _new_x: c_int,
        _m: Index,
        _g: *mut Number,
        _user_data: *mut c_void,
    ) -> c_int {
        1 // true
    }

    // No Jacobian (no constraints)
    extern "C" fn eval_jac_g(
        _n: Index,
        _x: *const Number,
        _new_x: c_int,
        _m: Index,
        _nele_jac: Index,
        _i_row: *mut Index,
        _j_col: *mut Index,
        _values: *mut Number,
        _user_data: *mut c_void,
    ) -> c_int {
        1 // true
    }

    // Hessian of Lagrangian (only objective, no constraints)
    extern "C" fn eval_h(
        _n: Index,
        _x: *const Number,
        _new_x: c_int,
        obj_factor: Number,
        _m: Index,
        _lambda: *const Number,
        _new_lambda: c_int,
        _nele_hess: Index,
        i_row: *mut Index,
        j_col: *mut Index,
        values: *mut Number,
        _user_data: *mut c_void,
    ) -> c_int {
        unsafe {
            if values.is_null() {
                // Return structure
                let irow_slice = std::slice::from_raw_parts_mut(i_row, 2);
                let jcol_slice = std::slice::from_raw_parts_mut(j_col, 2);
                irow_slice[0] = 0;
                jcol_slice[0] = 0; // (0,0)
                irow_slice[1] = 1;
                jcol_slice[1] = 1; // (1,1)
            } else {
                // Return values
                let values_slice = std::slice::from_raw_parts_mut(values, 2);
                values_slice[0] = obj_factor * 2.0; // d^2f/dx1^2
                values_slice[1] = obj_factor * 2.0; // d^2f/dx2^2
            }
        }
        1 // true
    }

    #[test]
    fn test_simple_optimization() {
        // Problem dimensions
        let n: Index = 2; // 2 variables
        let m: Index = 0; // 0 constraints

        // Variable bounds: -10 <= x1, x2 <= 10
        let x_l = vec![-10.0, -10.0];
        let x_u = vec![10.0, 10.0];

        // No constraints
        let g_l: Vec<Number> = vec![];
        let g_u: Vec<Number> = vec![];

        // Sparsity structure
        let nele_jac: Index = 0; // No Jacobian elements (no constraints)
        let nele_hess: Index = 2; // 2 elements in Hessian diagonal

        // Index style: 0 = C-style (0-based)
        let index_style: Index = 0;

        // Create the IPOPT problem
        let ipopt_problem = unsafe {
            CreateIpoptProblem(
                n,
                x_l.as_ptr(),
                x_u.as_ptr(),
                m,
                g_l.as_ptr(),
                g_u.as_ptr(),
                nele_jac,
                nele_hess,
                index_style,
                eval_f,
                eval_g,
                eval_grad_f,
                eval_jac_g,
                eval_h,
            )
        };

        assert!(!ipopt_problem.is_null(), "Failed to create IPOPT problem");

        // Set options
        unsafe {
            let tol_key = CString::new("tol").unwrap();
            AddIpoptNumOption(ipopt_problem, tol_key.as_ptr(), 1e-7);

            let print_level_key = CString::new("print_level").unwrap();
            AddIpoptIntOption(ipopt_problem, print_level_key.as_ptr(), 5);
        }

        // Starting point
        let mut x: Vec<f64> = vec![0.0, 1.0];
        let mut g: Vec<f64> = vec![0.0; m as usize];
        let mut obj_val: Number = 0.0;
        let mut mult_g: Vec<f64> = vec![0.0; m as usize];
        let mut mult_x_l: Vec<f64> = vec![0.0; n as usize];
        let mut mult_x_u: Vec<f64> = vec![0.0; n as usize];

        // Solve the problem
        let status = unsafe {
            IpoptSolve(
                ipopt_problem,
                x.as_mut_ptr(),
                g.as_mut_ptr(),
                &mut obj_val,
                mult_g.as_mut_ptr(),
                mult_x_l.as_mut_ptr(),
                mult_x_u.as_mut_ptr(),
                ptr::null_mut(),
            )
        };

        // Clean up
        unsafe {
            FreeIpoptProblem(ipopt_problem);
        }

        // Verify results
        assert_eq!(status, ApplicationReturnStatus::SolveSucceeded);
        assert!((x[0] - 1.0).abs() < 1e-5, "x[0] should be close to 1.0");
        assert!((x[1] - 2.5).abs() < 1e-5, "x[1] should be close to 2.5");
        assert!(
            obj_val.abs() < 1e-5,
            "Objective value should be close to 0.0"
        );
    }
}
