use std::rc::Rc;
use std::time::Instant;

use ray_tracing_rs::camera::Camera;
use ray_tracing_rs::hittable::{
    BvhNode, ConstantMedium, HitRecord, Hittable, MovingSphere, RectBox, RotateY, Sphere,
    Translate, XyRect, XzRect, YzRect,
};
use ray_tracing_rs::hittable_list::HittableList;
use ray_tracing_rs::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use ray_tracing_rs::ray::Ray;
use ray_tracing_rs::rtweekend::*;
use ray_tracing_rs::texture::{CheckerTexture, ConstantTexture, ImageTexture, NoiseTexture};
use ray_tracing_rs::vec3::Vec3;
fn main() {
    eprintln!("开始计时");
    let start = Instant::now(); // 开始计时
    let stdout = std::io::stdout();
    // let image_width = 1600;
    // let image_height = 800;
    // let samples_per_pixel = 100;
    // let max_depth = 50;
    // let image_width = 800;
    // let image_height = 800;
    // let samples_per_pixel = 200;
    // let max_depth = 50;
    let image_width = 600;
    let image_height = 600;
    let samples_per_pixel = 5000;
    let max_depth = 50;
    let aspect_ratio = image_width as f64 / image_height as f64;
    print!("P3\n{} {}\n255\n", image_width, image_height);
    let lookfrom = Vec3::from(278.0, 278.0, -800.0);
    let lookat = Vec3::from(278.0, 278.0, 0.0);
    let vup = Vec3::from(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let background = Vec3::new();
    let case = 7;
    let world = match case {
        1 => random_scene(),
        2 => two_spheres(),
        3 => earth(),
        4 => simple_light(),
        5 => cornell_box(),
        6 => cornell_smoke(),
        7 => final_scene(),
        _ => panic!(),
    };
    for j in (0..image_height).rev() {
        // eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            eprint!("\rPixels remaining: {}", (j + 1) * image_width - i);
            let mut color: Vec3 = Vec3::new();
            for _ in 0..samples_per_pixel {
                let u: f64 = (i as f64 + random_double()) / image_width as f64;
                let v: f64 = (j as f64 + random_double()) / image_height as f64;
                let r = cam.get_ray(u, v);
                color += ray_color(&r, background, &world, max_depth);
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

fn ray_color(r: &Ray, background: Vec3, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::from(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::new();
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return background;
    }

    let mut scattered: Ray = Ray::new();
    let mut attenuation: Vec3 = Vec3::new();
    let emitted = rec
        .material
        .as_deref()
        .unwrap()
        .emitted(rec.u, rec.v, &rec.p);
    if !rec
        .material
        .as_deref()
        .unwrap()
        .scatter(r, &rec, &mut attenuation, &mut scattered)
    {
        return emitted;
    }

    emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
}
fn final_scene() -> HittableList {
    let mut boxes1: HittableList = HittableList::default();
    let ground = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.48, 0.83, 0.53),
    ))));
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Rc::new(RectBox::from(
                Vec3::from(x0, y0, z0),
                Vec3::from(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    let mut objects: HittableList = HittableList::default();
    let boxes1_len = boxes1.objects.len();
    objects.add(Rc::new(BvhNode::from(
        &mut boxes1.objects,
        0,
        boxes1_len,
        0.0,
        1.0,
    )));
    let light = Rc::new(DiffuseLight::from(Rc::new(ConstantTexture::from(
        Vec3::from(7.0, 7.0, 7.0),
    ))));
    objects.add(Rc::new(XzRect::from(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));
    let center1 = Vec3::from(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::from(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.7, 0.3, 0.1),
    ))));
    objects.add(Rc::new(MovingSphere::from(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));
    objects.add(Rc::new(Sphere::from(
        Vec3::from(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::from(1.5)),
    )));
    objects.add(Rc::new(Sphere::from(
        Vec3::from(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::from(Vec3::from(0.8, 0.8, 0.9), 10.0)),
    )));
    let boundary = Rc::new(Sphere::from(
        Vec3::from(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::from(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Rc::new(ConstantMedium::from(
        boundary,
        0.2,
        Rc::new(ConstantTexture::from(Vec3::from(0.2, 0.4, 0.9))),
    )));
    let boundary = Rc::new(Sphere::from(
        Vec3::new(),
        5000.0,
        Rc::new(Dielectric::from(1.5)),
    ));
    objects.add(Rc::new(ConstantMedium::from(
        boundary,
        0.0001,
        Rc::new(ConstantTexture::from(Vec3::from(1.0, 1.0, 1.0))),
    )));
    let earth_img_texture = Rc::new(ImageTexture::from("imageTexture/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::from(earth_img_texture));
    objects.add(Rc::new(Sphere::from(
        Vec3::from(400.0, 200.0, 400.0),
        100.0,
        earth_surface,
    )));
    let pertext = Rc::new(NoiseTexture::from(0.1));
    objects.add(Rc::new(Sphere::from(
        Vec3::from(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian::from(pertext)),
    )));
    let mut boxes2: HittableList = HittableList::default();
    let white = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.73, 0.73, 0.73),
    ))));
    for _ in 0..1000 {
        boxes2.add(Rc::new(Sphere::from(
            Vec3::random_range(0.0, 165.0),
            10.0,
            white.clone(),
        )))
    }
    let boxes2_len = boxes2.objects.len();
    objects.add(Rc::new(Translate::from(
        Rc::new(RotateY::from(
            Rc::new(BvhNode::from(&mut boxes2.objects, 0, boxes2_len, 0.0, 1.0)),
            15.0,
        )),
        Vec3::from(-100.0, 270.0, 395.0),
    )));
    objects
}
fn cornell_smoke() -> HittableList {
    let mut world: HittableList = HittableList::default();
    let red = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.65, 0.05, 0.05),
    ))));
    let white = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.73, 0.73, 0.73),
    ))));
    let green = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.12, 0.45, 0.15),
    ))));
    let light = Rc::new(DiffuseLight::from(Rc::new(ConstantTexture::from(
        Vec3::from(7.0, 7.0, 7.0),
    ))));
    world.add(Rc::new(YzRect::from(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YzRect::from(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Rc::new(XzRect::from(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Rc::new(XzRect::from(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Rc::new(XyRect::from(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Rc::new(XzRect::from(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    let box1 = Rc::new(RectBox::from(
        Vec3::from(0.0, 0.0, 0.0),
        Vec3::from(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::from(box1, 15.0));
    let box1 = Rc::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    let box2 = Rc::new(RectBox::from(
        Vec3::from(0.0, 0.0, 0.0),
        Vec3::from(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(RotateY::from(box2, -18.0));
    let box2 = Rc::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(Rc::new(ConstantMedium::from(
        box1,
        0.01,
        Rc::new(ConstantTexture::from(Vec3::new())),
    )));
    world.add(Rc::new(ConstantMedium::from(
        box2,
        0.01,
        Rc::new(ConstantTexture::from(Vec3::from(1.0, 1.0, 1.0))),
    )));

    world
}
fn cornell_box() -> HittableList {
    let mut world: HittableList = HittableList::default();
    let red = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.65, 0.05, 0.05),
    ))));
    let white = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.73, 0.73, 0.73),
    ))));
    let green = Rc::new(Lambertian::from(Rc::new(ConstantTexture::from(
        Vec3::from(0.12, 0.45, 0.15),
    ))));
    let light = Rc::new(DiffuseLight::from(Rc::new(ConstantTexture::from(
        Vec3::from(15.0, 15.0, 15.0),
    ))));
    world.add(Rc::new(YzRect::from(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YzRect::from(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Rc::new(XzRect::from(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Rc::new(XzRect::from(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Rc::new(XyRect::from(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Rc::new(XzRect::from(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let box1 = Rc::new(RectBox::from(
        Vec3::from(0.0, 0.0, 0.0),
        Vec3::from(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::from(box1, 15.0));
    let box1 = Rc::new(Translate::from(box1, Vec3::from(265.0, 0.0, 295.0)));
    world.add(box1);
    let box2 = Rc::new(RectBox::from(
        Vec3::from(0.0, 0.0, 0.0),
        Vec3::from(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Rc::new(RotateY::from(box2, -18.0));
    let box2 = Rc::new(Translate::from(box2, Vec3::from(130.0, 0.0, 65.0)));
    world.add(box2);
    world
}
fn simple_light() -> HittableList {
    let mut world: HittableList = HittableList::default();
    let pertext = Rc::new(NoiseTexture::from(4.0));
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
    let difflight = Rc::new(DiffuseLight::from(Rc::new(ConstantTexture::from(
        Vec3::from(4.0, 4.0, 4.0),
    ))));
    world.add(Rc::new(Sphere::from(
        Vec3::from(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Rc::new(XyRect::from(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    world
}
fn earth() -> HittableList {
    let earth_img_texture = Rc::new(ImageTexture::from("imageTexture/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::from(earth_img_texture));
    let globe = Rc::new(Sphere::from(Vec3::from(0.0, 0.0, 0.0), 2.0, earth_surface));
    HittableList::new(globe)
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
    world
}
fn random_scene() -> HittableList {
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
