use std::rc::Rc;
use std::time::Instant;

use ray_tracing_rs::camera::Camera;
use ray_tracing_rs::hittable::{BvhNode, HitRecord, Hittable, MovingSphere, Sphere};
use ray_tracing_rs::hittable_list::HittableList;
use ray_tracing_rs::material::{Dielectric, Lambertian, Metal};
use ray_tracing_rs::ray::Ray;
use ray_tracing_rs::rtweekend::*;
use ray_tracing_rs::texture::{CheckerTexture, ConstantTexture, NoiseTexture};
use ray_tracing_rs::vec3::Vec3;
fn main() {
    eprintln!("开始计时");
    let start = Instant::now(); // 开始计时
    let stdout = std::io::stdout();
    let image_width = 1600;
    let image_height = 800;
    let samples_per_pixel = 100;
    let max_depth = 50;
    // let image_width = 400;
    // let image_height = 200;
    // let samples_per_pixel = 100;
    // let max_depth = 3;
    let aspect_ratio = image_width as f64 / image_height as f64;
    print!("P3\n{} {}\n255\n", image_width, image_height);
    let lookfrom = Vec3::from(13.0, 2.0, 3.0);
    let lookat = Vec3::from(0.0, 0.0, 0.0);
    let vup = Vec3::from(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    // let world = random_scene();
    let world = two_spheres();
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut color: Vec3 = Vec3::new();
            for _ in 0..samples_per_pixel {
                let u: f64 = (i as f64 + random_double()) / image_width as f64;
                let v: f64 = (j as f64 + random_double()) / image_height as f64;
                let r = cam.get_ray(u, v);
                color += ray_color(&r, &world, max_depth);
            }
            color
                .write_color(samples_per_pixel, &mut stdout.lock())
                .unwrap();
        }
    }
    eprintln!("\nDone.");
    let duration = start.elapsed(); // 停止计时并获取持续时间
    eprintln!("执行时间为: {:?}", duration); // 打印执行时间
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::from(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered: Ray = Ray::new();
        let mut attenuation: Vec3 = Vec3::new();
        if let Some(mat) = rec.material.clone() {
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
            return Vec3::from(0.0, 0.0, 0.0);
        }
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0)
}
fn two_spheres() -> HittableList {
    let mut world: HittableList = HittableList::default();
    //网格纹理
    // let checker = Rc::new(CheckerTexture::from(
    //     Rc::new(ConstantTexture::from(Vec3::from(0.2, 0.3, 0.1))),
    //     Rc::new(ConstantTexture::from(Vec3::from(0.9, 0.9, 0.9))),
    // ));
    //柏林噪声
    let pertext = Rc::new(NoiseTexture::new());
    world.add(Rc::new(Sphere::from(
        Vec3::from(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::from(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::from(
        Vec3::from(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::from(pertext)),
    )));
    // let len = world.objects.len();
    // HittableList::new(Rc::new(BvhNode::from(&mut world.objects, 0, len, 0.0, 1.0)))
    world
}

fn _random_scene() -> HittableList {
    let mut world: HittableList = HittableList::default();
    //网格纹理
    let checker = Rc::new(CheckerTexture::from(
        Rc::new(ConstantTexture::from(Vec3::from(0.2, 0.3, 0.1))),
        Rc::new(ConstantTexture::from(Vec3::from(0.9, 0.9, 0.9))),
    ));
    world.add(Rc::new(Sphere::from(
        Vec3::from(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::from(checker)),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3::from(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Vec3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    world.add(Rc::new(MovingSphere::from(
                        center,
                        center + Vec3::from(0.0, random_double_range(0.0, 0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(albedo)))),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    world.add(Rc::new(Sphere::from(
                        center,
                        0.2,
                        Rc::new(Metal::from(albedo, fuzz)),
                    )));
                } else {
                    // glass
                    world.add(Rc::new(Sphere::from(
                        center,
                        0.2,
                        Rc::new(Dielectric::from(1.5)),
                    )));
                }
            }
        }
    }
    world.add(Rc::new(Sphere::from(
        Vec3::from(0.0, 1.0, 0.0),
        1.0,
        Rc::new(Dielectric::from(1.5)),
    )));
    world.add(Rc::new(Sphere::from(
        Vec3::from(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
            Vec3::from(0.4, 0.2, 0.1),
        )))),
    )));
    world.add(Rc::new(Sphere::from(
        Vec3::from(4.0, 1.0, 0.0),
        1.0,
        Rc::new(Metal::from(Vec3::from(0.7, 0.6, 0.5), 0.0)),
    )));
    let len = world.objects.len();
    HittableList::new(Rc::new(BvhNode::from(&mut world.objects, 0, len, 0.0, 1.0)))
}
