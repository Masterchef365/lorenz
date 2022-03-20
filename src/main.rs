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
    //let anim = (time.cos() + 1.) / 2.;
    //let anim2 = ((time * 1.2).sin() + 1.) / 2.;
    //let anim3 = ((time * 1.7 + 2.32).cos() + 1.) / 2.;
    lorenz_lines(
        [1.01, 1., 1., 1., 1.],
        0.01,
        30_000,
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

fn lorenz_lines<const D: usize>(
    initial_pos: [f32; D],
    dt: f32,
    n: usize,
    _color: [f32; 3],
    scale: f32,
) -> Vec<Vertex> {
    let mut ode = RungeKutta::new(0., initial_pos, dt);

    ode.step(lorenz_96);

    std::iter::from_fn(|| {
        ode.step(lorenz_96);
        Some(ode.y())
    })
    .enumerate()
    .map(|(idx, pos): (usize, [f32; D])| {
        let idx = idx as f32;
        let i = idx / n as f32;
        let deriv = lorenz_96(0.0, pos);
        let vel = deriv.into_iter().map(|v| v * v).sum::<f32>().sqrt();
        Vertex::new(
            scalar_mul_nd(scale, trunc_3d(pos)),
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

fn lorenz_96<const D: usize>(_t: f32, x: [f32; D]) -> [f32; D] {
    let f = 8.;

    let mut deriv = [0.0; D];
    deriv.iter_mut().enumerate().for_each(|(i, d)| {
        *d = (x[(i + 1) % D] - x[(i - 2) % D]) * x[(i - 1) % D] - x[i] + f
    });

    deriv
}

pub struct RungeKutta<const D: usize> {
    x: f32,      // t
    y: [f32; D], // (x, y, z)
    step: f32,   // dt
}

impl<const D: usize> RungeKutta<D> {
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
                scalar_mul_nd(1. / 2., add_nd(self.y, k1)),
            ),
        );
        let k3 = scalar_mul_nd(
            self.step,
            f(
                self.x + self.step / 2.,
                scalar_mul_nd(1. / 2., add_nd(self.y, k2)),
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

fn add_nd<const D: usize>(mut a: [f32; D], b: [f32; D]) -> [f32; D] {
    a.iter_mut().zip(b).for_each(|(a, b)| *a += b);
    a
}

fn scalar_mul_nd<const D: usize>(scalar: f32, a: [f32; D]) -> [f32; D] {
    a.map(|v| v * scalar)
}

fn trunc_3d<const D: usize>(a: [f32; D]) -> [f32; 3] {
    let mut out = [0.; 3];
    out.copy_from_slice(&a[..3]);
    out
}
