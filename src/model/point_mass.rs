use simulation_toolbox::Model;

pub struct PointMass {
    // Properties
    name: String,
    // Parameters
    pub mass: f64,
}

impl PointMass {
    #[allow(dead_code)] // Allow unused constructor
    pub fn new(name: &str, mass: f64) -> Self {
        return PointMass {
            name: name.to_string(),
            mass,
        };
    }
}

impl Model for PointMass {
    fn name(&self) -> &str {
        return &self.name;
    }

    fn n_x(&self) -> usize {
        return 4; // [x, y, vx, vy]
    }

    fn n_u(&self) -> usize {
        return 2; // [Fx, Fy]
    }

    fn fun(&self, x: &Vec<f64>, u: &Vec<f64>, _t: f64) -> Vec<f64> {
        let mut dx: Vec<f64> = vec![0.0; self.n_x()];

        // dx/dt = v
        dx[0] = x[2];
        dx[1] = x[3];

        // dv/dt = F/m
        dx[2] = u[0] / self.mass;
        dx[3] = u[1] / self.mass;

        return dx;
    }

    fn jac(&self, _x: &Vec<f64>, _u: &Vec<f64>, _t: f64) -> Vec<f64> {
        let nx = self.n_x();
        let nu = self.n_u();
        let mut jac: Vec<f64> = vec![0.0; nx * (nx + nu)];

        // Jacobian is expected to be row major
        jac[0] = 0.0; // dx0/dx0
        jac[1] = 0.0; // dx0/dx1
        jac[2] = 1.0; // dx0/dx2
        jac[3] = 0.0; // dx0/dx3
        jac[4] = 0.0; // dx0/du0
        jac[5] = 0.0; // dx0/du1

        jac[6] = 0.0; // dx1/dx0
        jac[7] = 0.0; // dx1/dx1
        jac[8] = 0.0; // dx1/dx2
        jac[9] = 1.0; // dx1/dx3
        jac[10] = 0.0; // dx1/du0
        jac[11] = 0.0; // dx1/du1

        jac[12] = 0.0; // dx2/dx0
        jac[13] = 0.0; // dx2/dx1
        jac[14] = 0.0; // dx2/dx2
        jac[15] = 0.0; // dx2/dx3
        jac[16] = 1.0 / self.mass; // dx2/du0
        jac[17] = 0.0; // dx2/du1

        jac[18] = 0.0; // dx3/dx0
        jac[19] = 0.0; // dx3/dx1
        jac[20] = 0.0; // dx3/dx2
        jac[21] = 0.0; // dx3/dx3
        jac[22] = 0.0; // dx3/du0
        jac[23] = 1.0 / self.mass; // dx3/du1

        return jac;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_mass_dynamics() {
        let pm = PointMass::new("TestMass", 2.0);
        let x: Vec<f64> = vec![0.0, 0.0, 1.0, 1.0]; // Initial state
        let u: Vec<f64> = vec![2.0, 2.0]; // Input forces
        let t: f64 = 0.0;

        let dx = pm.fun(&x, &u, t);
        assert_eq!(dx, vec![1.0, 1.0, 1.0, 1.0]); // Expected derivatives
    }

    #[test]
    fn test_point_mass_jacobian() {
        let pm = PointMass::new("TestMass", 2.0);
        let x: Vec<f64> = vec![0.0, 0.0, 1.0, 1.0];
        let u: Vec<f64> = vec![2.0, 2.0];
        let t: f64 = 0.0;

        let jac = pm.jac(&x, &u, t);
        let expected_jac: Vec<f64> = vec![
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5,
        ];
        assert_eq!(jac, expected_jac); // Expected Jacobian
    }
}
