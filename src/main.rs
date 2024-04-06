use crate::raytracer::Raytracer;
use clap::Parser;
use std::num::{NonZeroU32, NonZeroUsize};
use std::path::PathBuf;

mod raytracer;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Number of iterations to run for each pixel
    #[arg(short, long, default_value_t = 64)]
    iterations: u32,
    /// Max number of bounces for each ray
    #[arg(short, long, default_value_t = 8)]
    bounces: u32,
    /// Width of the image to render
    #[arg(long, default_value_t = NonZeroU32::new(1920).unwrap())]
    width: NonZeroU32,
    /// Height of the image to render
    #[arg(long, default_value_t = NonZeroU32::new(1080).unwrap())]
    height: NonZeroU32,
    /// Optional path to output the file produced
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    /// Number of threads to use, will be determined based on number of CPUs if not specified
    #[arg(short, long)]
    threads: Option<NonZeroUsize>,
}

fn main() {
    let args = Cli::parse();
    if let Some(threads) = args.threads {
        rayon::ThreadPoolBuilder::default()
            .num_threads(threads.get())
            .build_global()
            .unwrap();
    }
    let mut raytracer = Raytracer::new(args.width.get(), args.height.get(), args.bounces);
    raytracer.run(args.iterations);
    let out_path = args.output.unwrap_or(PathBuf::from("image.png"));
    raytracer
        .image()
        .write_file(&out_path)
        .expect("Failed to save image file");
}
