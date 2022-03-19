use idek::{prelude::*, IndexBuffer, MultiPlatformCamera};

fn main() -> Result<()> {
    launch::<_, TriangleApp>(Settings::default().vr_if_any_args())
}

struct TriangleApp {
    verts: VertexBuffer,
    indices: IndexBuffer,
    camera: MultiPlatformCamera,
    lines_shader: Shader,
}

impl App for TriangleApp {
    fn init(ctx: &mut Context, platform: &mut Platform, _: ()) -> Result<Self> {
        let (vertices, indices) = lorenz_lines(
            [1., 1., 1.], 
            [10., 28., 8. / 3.], 
            0.01, 
            40, 
            [1.; 3],
            1. / 10.,
        );
        dbg!(&vertices);

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
    initial_pos: [f32; 3],
    coeffs: [f32; 3],
    dt: f32,
    n: usize,
    color: [f32; 3],
    scale: f32,
) -> (Vec<Vertex>, Vec<u32>) {
    let vertices: Vec<Vertex> = lorenz(initial_pos, coeffs, dt)
        .map(|pos| pos.map(|v| v * scale))
        .map(|pos| Vertex::new(pos, color))
        .take(n)
        .collect();
    let indices: Vec<u32> = (0..)
        .map(|i| (i + 1) / 2)
        .take((vertices.len() - 1) * 2)
        .collect();
    (vertices, indices)
}

fn lorenz(
    [mut x, mut y, mut z]: [f32; 3],
    [sigma, rho, beta]: [f32; 3],
    dt: f32,
) -> impl Iterator<Item = [f32; 3]> {
    std::iter::from_fn(move || {
        let next_x = dt * sigma * (x - y);
        let next_y = dt * (x * (rho - z) - y);
        z = dt * (x * y - beta * z);
        x = next_x;
        y = next_y;
        Some([x, y, z])
    })
}
