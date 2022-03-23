use anyhow::Result;
use lorenz::*;
use std::{cmp::Ordering, fs::File, io::BufWriter};
use structopt::StructOpt;
use wav::{BitDepth, Header, WAV_FORMAT_IEEE_FLOAT};

#[derive(StructOpt, Debug)]
struct Opt {
    /*
    /// Coefficients
    #[structopt(short, long, default_value = "1.0", use_delimiter = true)]
    coeffs: Vec<f32>,

    /// Use the lorenz96 model instead of the lorenz attractor
    #[structopt(short, long)]
    lorenz96: bool,
    */
    /// Time step
    #[structopt(short, long, default_value = "0.01")]
    dt: f32,

    /// Total audio time in seconds (may not match the 'dt' for the lorenz system). Used to
    /// calculate the number of desired total samples
    #[structopt(short, long, default_value = "2.0")]
    total_time: f32,

    /// Audio sampling rate
    #[structopt(short, long, default_value = "48000")]
    sampling_rate: u32,
}

fn main() -> Result<()> {
    let args = Opt::from_args();

    let total_samples = (args.total_time * args.sampling_rate as f32) as usize;

    let f = 8.;
    let initial_pos = [f, f - 0.01, f, f + 0.01, f];
    let forcing = [f, f, f, f + 1.2, f];
    let mut ode = RungeKutta::new(0., initial_pos, args.dt);
    let lorenz_96 = |_, pos| lorenz_96(pos, forcing);

    let mut samples = Vec::with_capacity(total_samples);
    for _ in 0..total_samples {
        ode.step(lorenz_96);
        samples.push(ode.y());
    }

    let header = Header::new(WAV_FORMAT_IEEE_FLOAT, 1, args.sampling_rate, 32);

    for idx in 0..initial_pos.len() {
        let c = (b'a' + (idx as u8 + (b'x' - b'a')) % 26) as char;

        let filename = format!("{c}.wav");
        let mut data: Vec<f32> = samples.iter().map(|sample| sample[idx]).collect();

        // Normalize data
        let max_entry = data
            .iter()
            .copied()
            .max_by(|a, b| a.partial_cmp(&b).unwrap_or(Ordering::Equal))
            .expect("No data");
        data = data.into_iter().map(|d| d / max_entry).collect();

        let data = BitDepth::ThirtyTwoFloat(data);
        let mut file = BufWriter::new(File::create(filename)?);
        wav::write(header, &data, &mut file)?;
    }

    Ok(())
}
