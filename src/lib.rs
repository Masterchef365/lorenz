use ultraviolet::Vec3;

pub fn lorenz([x, y, z]: [f32; 3], [sigma, rho, beta]: [f32; 3]) -> [f32; 3] {
    [sigma * (y - x), x * (rho - z) - y, x * y - beta * z]
}

pub struct RungeKutta {
    x: f32,    // t
    y: Vec3,   // (x, y, z)
    step: f32, // dt
}

impl RungeKutta {
    pub fn new(x_min: f32, y_init: Vec3, step: f32) -> Self {
        Self {
            x: x_min,
            y: y_init,
            step,
        }
    }

    /// Returns the result of fourth-order Runge-Kutta method for a given function
    /// f(x, y) -> dy/dx
    pub fn step(&mut self, f: impl Fn(f32, Vec3) -> Vec3) {
        let k1 = self.step * f(self.x, self.y);
        let k2 = self.step * f(self.x + self.step / 2., self.y + k1 / 2.);
        let k3 = self.step * f(self.x + self.step / 2., self.y + k2 / 2.);
        let k4 = self.step * f(self.x + self.step, self.y + k3);
        self.y += (k1 + 2. * k2 + 2. * k3 + k4) / 6.;
        self.x += self.step;
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> Vec3 {
        self.y
    }
}

