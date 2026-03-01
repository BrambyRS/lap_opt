mod ipopt;
mod problem_data;

// IPOPT callback helper functions
fn get_node_state(x: &[f64], nx: usize, nu: usize, i: usize) -> Vec<f64> {
    return x[i*(nx + nu)..i*(nx + nu) + nx].to_vec();
}

fn get_node_control(x: &[f64], nx: usize, nu: usize, i: usize) -> Vec<f64> {
    return x[i*(nx + nu) + nx..(i+1)*(nx + nu)].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
