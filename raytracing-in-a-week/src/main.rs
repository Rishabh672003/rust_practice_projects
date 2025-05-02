use rand::Rng;
use std::{error::Error, fs::File, io::Write};

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

mod hitable;
use hitable::{Hitable, HitableList};

mod sphere;
use sphere::Sphere;

mod camera;
use camera::Camera;

fn random_in_unit_sphere() -> Vec3 {
    let unit = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = rand::rng();
    loop {
        let p =
            2.0 * Vec3::new(
                rng.random::<f64>(),
                rng.random::<f64>(),
                rng.random::<f64>(),
            ) - unit;
        if p.squared_length() < 1.0 {
            break p;
        }
    }
}

fn color(ray: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        let target = hit.p - hit.normal + random_in_unit_sphere();
        0.5 * color(&Ray::new(hit.p, target - hit.p), world)
    } else {
        let unit_dir = ray.direction().unit_vec();
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let cam = Camera::new();
    let mut rng = rand::rng();

    let mut image = String::new();
    image.push_str(&format!("P3\n{nx} {ny}\n255\n"));
    let world = Box::new(HitableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]));

    for j in (0..=ny - 1).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f64 + rng.random::<f64>()) / nx as f64;
                let v = (j as f64 + rng.random::<f64>()) / ny as f64;
                let r = cam.get_ray(u, v);
                let _ = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }
            col /= ns as f64;
            col = col.sqrt();
            let ir = (255.99 * col[0]) as u8;
            let ig = (255.99 * col[1]) as u8;
            let ib = (255.99 * col[2]) as u8;
            image.push_str(&format!("{ir} {ig} {ib}\n"));
        }
    }
    let mut image_file = File::create("images/chapter_7.2.ppm")?;
    let _ = image_file.write(image.as_bytes());
    Ok(())
}
