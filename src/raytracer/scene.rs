use nalgebra::{UnitVector3, Vector2, Vector3};

pub struct Scene {
    camera: Camera,
}

struct Camera {
    viewport: Vector2<f32>,
    resolution: Vector2<u32>,
    center: Vector3<f32>,
    focal_length: f32,
}

pub struct Ray {
    origin: Vector3<f32>,
    direction: UnitVector3<f32>,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            camera: Camera {
                viewport: Vector2::new(2.0f32 * (width / height) as f32, 2.0f32),
                resolution: Vector2::new(width, height),
                center: Vector3::new(0., 0., 0.),
                focal_length: 1.,
            },
        }
    }

    pub fn fire_ray(&self, x: usize, y: usize) -> Ray {
        let u = Vector3::new(self.camera.viewport.x, 0., 0.);
        let v = Vector3::new(0., -self.camera.viewport.y, 0.);
        let delta_u = u / self.camera.resolution.x as f32;
        let delta_v = v / self.camera.resolution.y as f32;
        let upper_left =
            self.camera.center - Vector3::new(0., 0., self.camera.focal_length) - u / 2. - v / 2.;
        let loc00 = upper_left + 0.5 * (delta_u + delta_v);
        let center = loc00 + (x as f32 * delta_u) + (y as f32 * delta_v);
        let direction = UnitVector3::new_normalize(center - self.camera.center);
        Ray {
            origin: self.camera.center,
            direction,
        }
    }
}

impl Ray {
    pub fn color(&self) -> Vector3<f32> {
        let a = 0.5 * (self.direction.y + 1.);
        (1. - a) * Vector3::new(1., 1., 1.) + a * Vector3::new(0.5, 0.7, 1.)
    }
}
