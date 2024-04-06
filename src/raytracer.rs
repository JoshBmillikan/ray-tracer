use crate::raytracer::scene::Scene;
use nalgebra::{Vector2, Vector3, Vector4};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::time::Instant;

mod scene;

pub struct Raytracer {
    image: Image,
    scene: Scene,
    max_bounces: u32,
}

pub struct Image {
    pixels: Box<[Vector4<f32>]>,
    extent: Vector2<u32>,
}

impl Raytracer {
    pub fn new(width: u32, height: u32, max_bounces: u32) -> Self {
        let image = Image::new(width, height);

        Self {
            image,
            scene: Scene::new(width, height),
            max_bounces,
        }
    }

    pub fn run(&mut self, iterations: u32) {
        println!(
            "Raytracing image of {}x{}",
            self.image.extent.x, self.image.extent.y
        );
        let start = Instant::now();
        for i in 0..iterations {
            self.image.run_iteration(|x, y| {
                let ray = self.scene.fire_ray(x, y);
                let color = ray.color();
                Vector4::new(color.x, color.y, color.z, 1.)
            });
            let percent = ((i + 1) as f32 / iterations as f32) * 100.;
            println!("{percent:.2}%");
        }
        self.image
            .pixels
            .iter_mut()
            .for_each(|p| *p /= iterations as f32);
        let finish = Instant::now();
        println!("Finished in {:.3} seconds", (finish - start).as_secs_f64())
    }

    pub fn image(&self) -> &Image {
        &self.image
    }
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        let extent = Vector2::new(width, height);
        let pixels = vec![Default::default(); (width * height) as usize].into_boxed_slice();
        Self { pixels, extent }
    }

    fn run_iteration(&mut self, func: impl Fn(usize, usize) -> Vector4<f32> + Send + Sync) {
        let chunk_size =
            (self.extent.x * self.extent.y) as usize / rayon::current_num_threads() + 1;
        self.pixels
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(chunk_index, chunk)| {
                let mut i = chunk_index * chunk_index;
                for pixel in chunk {
                    let x = i % self.extent.x as usize;
                    let y = i / self.extent.y as usize;
                    *pixel += func(x, y);
                    i += 1;
                }
            });
    }

    pub fn write_file(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let data = self.pixels.iter().fold(
            Vec::with_capacity((self.extent.x * self.extent.y * 4) as usize),
            |mut data, pixel| {
                data.push((u8::MAX as f32 * pixel.x).round() as u8);
                data.push((u8::MAX as f32 * pixel.y).round() as u8);
                data.push((u8::MAX as f32 * pixel.z).round() as u8);
                data.push((u8::MAX as f32 * pixel.w).round() as u8);
                data
            },
        );
        let file = File::create(path)?;
        let writer = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(writer, self.extent.x, self.extent.y);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&data)?;

        Ok(())
    }
}

impl Index<Vector2<u32>> for Image {
    type Output = Vector4<f32>;

    fn index(&self, index: Vector2<u32>) -> &Self::Output {
        &self.pixels[self.extent.x as usize * index.y as usize + index.x as usize]
    }
}

impl IndexMut<Vector2<u32>> for Image {
    fn index_mut(&mut self, index: Vector2<u32>) -> &mut Self::Output {
        &mut self.pixels[self.extent.x as usize * index.y as usize + index.x as usize]
    }
}
