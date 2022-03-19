use idek::{prelude::*, IndexBuffer, MultiPlatformCamera};
use ultraviolet::Vec3;

fn main() -> Result<()> {
    launch::<_, LorenzViz>(Settings::default().vr_if_any_args().msaa_samples(8))
}

struct LorenzViz {
    verts: VertexBuffer,
    indices: IndexBuffer,
    camera: MultiPlatformCamera,
    lines_shader: Shader,
}

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1. - t) + b * t
}

fn lorenz_with_time(time: f32) -> Vec<Vertex> {
    let anim = (time.cos() + 1.) / 2.;
    let anim2 = ((time * 1.2).sin() + 1.) / 2.;
    let anim3 = ((time * 1.7 + 2.32).cos() + 1.) / 2.;
    lorenz_lines(
        [1., 1., 1.].into(),
        [
            //mix(0.5, 1., anim3) * 10.,
            //mix(0.5, 1., anim2) * 28.,
            //anim * 8. / 3.,
            10.,
            28.,
            8. / 3.,
        ],
        0.005,
        300_000,
        [1.; 3],
        1. / 10.,
    )
}

impl App for LorenzViz {
    fn init(ctx: &mut Context, platform: &mut Platform, _: ()) -> Result<Self> {
        let vertices = lorenz_with_time(0.);
        let indices = line_strip_indices(vertices.len());

        Ok(Self {
            verts: ctx.vertices(&vertices, false)?,
            indices: ctx.indices(&indices, false)?,
            lines_shader: ctx.shader(
                DEFAULT_VERTEX_SHADER,
                &std::fs::read("./shaders/unlit.frag.spv")?,
                Primitive::Lines,
            )?,
            camera: MultiPlatformCamera::new(platform),
        })
    }

    fn frame(&mut self, ctx: &mut Context, _: &mut Platform) -> Result<Vec<DrawCmd>> {
        //let vertices = lorenz_with_time(ctx.start_time().elapsed().as_secs_f32());
        //ctx.update_vertices(self.verts, &vertices)?;

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
    _color: [f32; 3],
    scale: f32,
) -> Vec<Vertex> {
    let mut ode = RungeKutta::new(0., initial_pos, dt);

    let f = |_, pos: Vec3| lorenz(pos.into(), coeffs).into();

    ode.step(f);

    std::iter::from_fn(|| {
        ode.step(f);
        Some(ode.y().into())
    })
    .enumerate()
    .map(|(idx, pos): (usize, [f32; 3])| {
        let idx = idx as f32;
        let i = idx / n as f32;
        let deriv = lorenz(pos, coeffs);
        let vel = deriv.into_iter().map(|v| v * v).sum::<f32>().sqrt();
        Vertex::new(
            pos.map(|v| v * scale),
            [i, idx, vel],
            //lorenz(pos, coeffs).map(|v| v.abs().sqrt() * scale),
        )
    })
    .take(n)
    .collect()
}

fn line_strip_indices(n: usize) -> Vec<u32> {
    (0..).map(|i| (i + 1) / 2).take((n - 1) * 2).collect()
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
