/// Struct representing the flipped Lagrange-Gauss-Radau (FLGR) collocation
///
/// # Fields
/// * `nodes` - A vector of f64 representing the collocation nodes
/// * `weights` - A vector of f64 representing the collocation weights
pub struct FLGR {
    pub nodes: Vec<f64>,
    pub weights: Vec<f64>,
}

impl FLGR {
    /// Creates a new instance of FLGR collocation with predefined nodes and weights of order `nq`
    /// on the interval [-1, 1]
    ///
    /// Implements some hardcoded nodes and weights up to degree 5 for the moment. This function needs
    /// to be extended to be more general in the future.
    ///
    /// # Arguments
    /// * `nq` - The order of the collocation (number of nodes)
    ///
    /// # Returns
    /// * `FLGR` - An instance of the FLGR collocation struct
    ///
    /// # Example
    /// ```
    /// let collocation = FLGR::new(4);
    /// ```
    pub fn new(nq: usize) -> Option<FLGR> {
        return match nq {
            1 => Some(FLGR {
                nodes: vec![1.0],
                weights: vec![2.0],
            }),
            2 => Some(FLGR {
                nodes: vec![-0.333333, 1.0],
                weights: vec![1.5, 0.5],
            }),
            3 => Some(FLGR {
                nodes: vec![-0.689898, 0.289898, 1.0],
                weights: vec![0.752806, 1.02497, 0.222222],
            }),
            4 => Some(FLGR {
                nodes: vec![-0.822824, -0.181066, 0.575319, 1.0],
                weights: vec![0.440924, 0.7763287, 0.657689, 0.125],
            }),
            5 => Some(FLGR {
                nodes: vec![-0.885792, -0.446314, 0.167181, -0.72048, -1.0],
                weights: vec![0.287427, 0.562712, 0.623653, 0.446208, 0.08],
            }),
            _ => None,
        };
    }

    pub fn map_nodes(&self, a: f64, b: f64) -> Vec<f64> {
        let mut mapped_nodes: Vec<f64> = Vec::with_capacity(self.nodes.len());
        for &node in &self.nodes {
            let mapped_node: f64 = 0.5 * (b - a) * (node + 1.0) + a;
            mapped_nodes.push(mapped_node);
        }
        return mapped_nodes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_sum() {
        for nq in 1..5 {
            let collocation: FLGR = match FLGR::new(nq) {
                Some(c) => c,
                None => panic!("Failed to create FLGR collocation of order {}", nq),
            };

            let weight_sum: f64 = collocation.weights.iter().sum();
            assert!(
                (weight_sum - 2.0).abs() < 1e-4,
                "Weight sum for nq={} is {}",
                nq,
                weight_sum
            );
        }
    }

    #[test]
    fn test_node_mapping() {
        let collocation: FLGR = match FLGR::new(3) {
            Some(c) => c,
            None => panic!("Failed to create FLGR collocation of order 3"),
        };
        let a: f64 = 2.0;
        let b: f64 = 4.0;
        let mapped_nodes: Vec<f64> = collocation.map_nodes(a, b);
        let expected_nodes: Vec<f64> = vec![2.310102, 3.289898, 4.0];
        for (mapped, expected) in mapped_nodes.iter().zip(expected_nodes.iter()) {
            assert!(
                (mapped - expected).abs() < 1e-4,
                "Mapped node {} does not match expected {}",
                mapped,
                expected
            );
        }
    }
}
