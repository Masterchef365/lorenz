use idek::{prelude::*, IndexBuffer, MultiPlatformCamera};
use lorenz::*;

fn main() -> Result<()> {
    launch::<_, LorenzViz>(Settings::default().vr_if_any_args().msaa_samples(8))
}

struct LorenzViz {
    verts: VertexBuffer,
    indices: IndexBuffer,
    camera: MultiPlatformCamera,
    lines_shader: Shader,
}

/*fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1. - t) + b * t
}*/

fn lorenz_with_time(_time: f32) -> Vec<Vertex> {
    //let time = time / 100.;
    //let anim = (time.sin() + 1.) / 2.;

    let perturb = 0.01;

    lorenz_lines(
        //[8.001, 8., 8., 8., 8.],
        [8., 8., 8., 8., 8. - perturb],
        0.01,
        300_000,
        [1.; 3],
        1. / 10.,
    )
}

const ANIMATE: bool = false;

impl App for LorenzViz {
    fn init(ctx: &mut Context, platform: &mut Platform, _: ()) -> Result<Self> {
        let vertices = lorenz_with_time(0.);
        let indices = line_strip_indices(vertices.len());

        Ok(Self {
            verts: ctx.vertices(&vertices, ANIMATE)?,
            indices: ctx.indices(&indices, false)?,
            lines_shader: ctx.shader(
                DEFAULT_VERTEX_SHADER,
                &std::fs::read("./shaders/unlit.frag.spv")?,
                Primitive::Lines,
                Blend::Additive,
            )?,
            camera: MultiPlatformCamera::new(platform),
        })
    }

    fn frame(&mut self, ctx: &mut Context, _: &mut Platform) -> Result<Vec<DrawCmd>> {
        if ANIMATE {
            let vertices = lorenz_with_time(ctx.start_time().elapsed().as_secs_f32());
            ctx.update_vertices(self.verts, &vertices)?;
        }

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

    let lorenz_96 = |_, pos| lorenz_96(pos, 8.);

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

