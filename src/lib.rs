pub fn lorenz([x, y, z]: [f32; 3], [sigma, rho, beta]: [f32; 3]) -> [f32; 3] {
    [sigma * (y - x), x * (rho - z) - y, x * y - beta * z]
}

pub fn lorenz_96<const D: usize>(x: [f32; D], f: f32) -> [f32; D] {
    let mut deriv = [0.0; D];
    deriv.iter_mut().enumerate().for_each(|(i, d)| {
        let wrap = |k: i32| {
            let idx = i as i32 + k;
            if idx < 0 {
                (D as i32 + idx) as usize
            } else {
                idx as usize % D
            }
        };
        *d = (x[wrap(1)] - x[wrap(-2)]) * x[wrap(-1)] - x[i] + f
    });

    deriv
}

/// N-dimensional fourth-order Runge-Kutta ODE solver
pub struct RungeKutta<const D: usize> {
    x: f32,      // t
    y: [f32; D], // (x, y, z, ...)
    step: f32,   // dt
}

impl<const D: usize> RungeKutta<D> {
    /// Create a new solver with initial conditions and step size
    pub fn new(x_min: f32, y_init: [f32; D], step: f32) -> Self {
        Self {
            x: x_min,
            y: y_init,
            step,
        }
    }

    /// Returns the result of fourth-order Runge-Kutta method for a given function
    /// f(x, y) -> dy/dx
    pub fn step(&mut self, f: impl Fn(f32, [f32; D]) -> [f32; D]) {
        let k1 = scalar_mul_nd(self.step, f(self.x, self.y));
        let k2 = scalar_mul_nd(
            self.step,
            f(
                self.x + self.step / 2.,
                add_nd(self.y, scalar_mul_nd(1. / 2., k1)),
            ),
        );
        let k3 = scalar_mul_nd(
            self.step,
            f(
                self.x + self.step / 2.,
                add_nd(self.y, scalar_mul_nd(1. / 2., k2)),
            ),
        );
        let k4 = scalar_mul_nd(self.step, f(self.x + self.step, add_nd(self.y, k3)));
        self.y = add_nd(
            self.y,
            scalar_mul_nd(
                1. / 6.,
                add_nd(
                    add_nd(k1, scalar_mul_nd(2., k2)),
                    add_nd(scalar_mul_nd(2., k3), k4),
                ),
            ),
        );
        self.x += self.step;
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> [f32; D] {
        self.y
    }
}

/// Add vectors
pub fn add_nd<const D: usize>(mut a: [f32; D], b: [f32; D]) -> [f32; D] {
    a.iter_mut().zip(b).for_each(|(a, b)| *a += b);
    a
}

/// Scalar multiply a vector
pub fn scalar_mul_nd<const D: usize>(scalar: f32, a: [f32; D]) -> [f32; D] {
    a.map(|v| v * scalar)
}

/// Truncate a vector to 3D, and fill the remainder with zeroes
pub fn trunc_3d<const D: usize>(a: [f32; D]) -> [f32; 3] {
    let mut out = [0.; 3];
    out.copy_from_slice(&a[..3]);
    out
}
