pub mod point_mass;

/*
Get the scaling factor dt/ds for a model to convert time derivatives to spatial derivatives.
This assumes states 3 and 4 of the model are always the x- and y-velocities respectively.
*/

pub fn get_scaling(x: Vec<f64>, tangent: Vec<f64>) -> f64 {
    // Get tangent components
    let tx: f64 = tangent[0];
    let ty: f64 = tangent[1];
    // Get velocity components
    let vx: f64 = x[2];
    let vy: f64 = x[3];
    // Caclulate speed along tangent
    let v_slap: f64 = vx * tx + vy * ty;
    let ds_dt: f64 = 1.0 / v_slap;
    return ds_dt;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaling_pos_x() {
        let x: Vec<f64> = vec![0.0, 0.0, 3.0, 0.0];
        let tangent: Vec<f64> = vec![1.0, 0.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!((ds_dt - 1.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_scaling_pos_y() {
        let x: Vec<f64> = vec![0.0, 0.0, 0.0, 4.0];
        let tangent: Vec<f64> = vec![0.0, 1.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!((ds_dt - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_scaling_neg_x() {
        let x: Vec<f64> = vec![0.0, 0.0, -2.0, 0.0];
        let tangent: Vec<f64> = vec![1.0, 0.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!((ds_dt + 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_scaling_neg_y() {
        let x: Vec<f64> = vec![0.0, 0.0, 0.0, -5.0];
        let tangent: Vec<f64> = vec![0.0, 1.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!((ds_dt + 0.2).abs() < 1e-6);
    }

    #[test]
    fn test_scaling_pos_diag() {
        let x: Vec<f64> = vec![0.0, 0.0, 2.0_f64.sqrt(), 2.0_f64.sqrt()];
        let tangent: Vec<f64> = vec![2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!(ds_dt - 0.5 < 1e-6);
    }

    #[test]
    fn test_scaling_neg_diag() {
        let x: Vec<f64> = vec![0.0, 0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0];
        let tangent: Vec<f64> = vec![2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!((ds_dt + 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_scaling_angled() {
        let x: Vec<f64> = vec![0.0, 0.0, 1.0, 0.5];
        let tangent: Vec<f64> = vec![1.0, 0.0, 0.0];
        let ds_dt: f64 = get_scaling(x, tangent);
        assert!((ds_dt - 1.0).abs() < 1e-6);
    }
}
