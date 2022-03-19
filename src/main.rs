use idek::{prelude::*, IndexBuffer, MultiPlatformCamera};
use ultraviolet::Vec3;

fn main() -> Result<()> {
    launch::<_, LorenzViz>(Settings::default().vr_if_any_args())
}

struct LorenzViz {
    verts: VertexBuffer,
    indices: IndexBuffer,
    camera: MultiPlatformCamera,
    lines_shader: Shader,
}

impl App for LorenzViz {
    fn init(ctx: &mut Context, platform: &mut Platform, _: ()) -> Result<Self> {
        let (vertices, indices) = lorenz_lines(
            [1., 1., 1.].into(),
            [10., 28., 8. / 3.],
            0.01,
            4000,
            [1.; 3],
            1. / 10.,
        );

        Ok(Self {
            verts: ctx.vertices(&vertices, false)?,
            indices: ctx.indices(&indices, false)?,
            lines_shader: ctx.shader(
                DEFAULT_VERTEX_SHADER,
                DEFAULT_FRAGMENT_SHADER,
                Primitive::Lines,
            )?,
            camera: MultiPlatformCamera::new(platform),
        })
    }

    fn frame(&mut self, _ctx: &mut Context, _: &mut Platform) -> Result<Vec<DrawCmd>> {
        Ok(vec![DrawCmd::new(self.verts)
            .indices(self.indices)
            .shader(self.lines_shader)])
    }

    fn event(
        &mut self,
        ctx: &mut Context,
        platform: &mut Platform,
        mut event: Event,
    ) -> Result<()> {
        if self.camera.handle_event(&mut event) {
            ctx.set_camera_prefix(self.camera.get_prefix())
        }
        idek::close_when_asked(platform, &event);
        Ok(())
    }
}

fn lorenz_lines(
    initial_pos: Vec3,
    coeffs: [f32; 3],
    dt: f32,
    n: usize,
    color: [f32; 3],
    scale: f32,
) -> (Vec<Vertex>, Vec<u32>) {
    let mut ode = RungeKutta::new(0., initial_pos, dt);

    let f = |_, pos: Vec3| lorenz(pos.into(), coeffs).into();

    ode.step(f);

    let vertices: Vec<Vertex> = std::iter::from_fn(|| {
        ode.step(f);
        Some(ode.y().into())
    })
    .map(|pos: [f32; 3]| pos.map(|v| v * scale))
    .map(|pos| Vertex::new(pos, color))
    .take(n)
    .collect();
    let indices: Vec<u32> = (0..)
        .map(|i| (i + 1) / 2)
        .take((vertices.len() - 1) * 2)
        .collect();
    (vertices, indices)
}

fn lorenz([x, y, z]: [f32; 3], [sigma, rho, beta]: [f32; 3]) -> [f32; 3] {
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
