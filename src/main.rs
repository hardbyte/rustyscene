use std::ops::{Mul, Range};
use itertools::Itertools;
use indicatif::{ProgressIterator};
use image::{GenericImageView, RgbImage, Pixel, Rgb};
use rayon::prelude::*;

use rand::{random, Rng};
use crate::camera::Camera;
use crate::dielectric::Dielectric;
use crate::diffuse::Diffuse;
use crate::hit::{HitRecord, Hittable};
use crate::material::{Material, Scatter};
use crate::metal::Metal;
use crate::objects::sphere::Sphere;
use crate::objects::triangle::Triangle;
use crate::primitives::{Color, Direction, NumericType, Point, Time, Vector};
use crate::ray::Ray;

pub mod objects;

pub mod primitives;
pub mod ray;
pub mod hit;
mod camera;
mod utils;
mod material;
mod diffuse;
mod metal;
mod dielectric;

struct ImageProperties {
    height: u32,
    width: u32,
}

const DEFAULT_IMAGE_PROPERTIES: ImageProperties = ImageProperties { width: 320, height: 300 };
const MAX_VALUE: u8 = 255;

const DEFAULT_BG_COLOR: Color = Color::new(0.6, 0.6, 0.5);


enum ObjectTypes {
    Sphere(Sphere),
    Triangle(Triangle)
}
impl ObjectTypes {
    fn new_sphere(center: Point, radius: f64, material: Material) -> Self {
        ObjectTypes::Sphere(Sphere {
            center,
            radius,
            material
        })
    }

    fn new_triangle(p0: Point, p1: Point, p2: Point, material: Material) -> Self {
        ObjectTypes::Triangle(Triangle::new(p0, p1, p2, material))
    }
}
type Scene = Vec<ObjectTypes>;

//type Scene<'a> = Vec<Box<dyn Hittable<'a>>>;
pub struct RenderOptions {
    samples: u32,
    max_bounces: u32,
}

fn hit<'a>(objects: &'a Scene, ray: &'a Ray, interval: &Range<Time>) -> Option<HitRecord<'a>> {
    let mut closest_so_far = interval.end;
    let mut closest_hit: Option<HitRecord> = None;

    for obj in objects {

        let current_hit = match obj {
            ObjectTypes::Sphere(sphere) => sphere.hits(ray, &Range { start: interval.start, end: closest_so_far }),
            ObjectTypes::Triangle(triangle) => triangle.hits(ray, &Range { start: interval.start, end: closest_so_far }),
        };

        if let Some(hit) = current_hit {
            if hit.t < closest_so_far {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }
    }

    closest_hit
}


fn color<'a>(ray: &mut Ray, objects: &Scene, max_bounces: u32) -> Color {
    let mut output_attenuation = Vector::new(1.0, 1.0, 1.0);
    let black = Color::new(0.0, 0.0, 0.0);

    for depth in 0..max_bounces {
        // Get the first object intersection
        let interval: Range<Time> = Range { start: 0.0001, end: Time::MAX };
        let cloned_ray = ray.clone();
        let hit_record_option = hit(objects, &cloned_ray, &interval);


        if let Some(hit) = hit_record_option {
            // Hit an object!
            let scatter_record = hit.material.scatter(ray, &hit);
            match scatter_record.scattered {
                false => {
                    // hit object has absorbed the ray e.g a light source or edge of a sphere
                    // todo emissions

                    return black;
                }
                true => {
                    // Update the color and follow the new ray
                    output_attenuation = output_attenuation.mul(scatter_record.attenuation);
                }
            }

        } else {
            // Missed all objects, sample the "background" sky.
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * unit_direction.y + 1.0;
            let bg: Color = (1.0 - a) * DEFAULT_BG_COLOR + a * Color::new(0.5, 0.7, 1.0);

            return output_attenuation.mul(bg);
        }
    }

    // Return black after max bounces
    return Color::new(0.0, 0.0, 0.0);
}

fn raytrace<'a>(camera: Camera, scene: &Scene, config: RenderOptions, output: &mut RgbImage) {
    let width = output.width();
    let height = output.height();

    // TODO look at rayon par_iter here
    for row in 0..height {
        for col in 0..width {

            let sample_range = 0..config.samples;

            let pixel: Color = sample_range.into_par_iter().map(|sample_number| {
                    let mut rng = rand::thread_rng();
                    let u = (rng.gen::<NumericType>() + col as NumericType) / width as NumericType;
                    let v = (rng.gen::<NumericType>() + row as NumericType) / height as NumericType;
                    let mut ray = camera.get_ray(u, v);

                    // Color the ray
                    color(&mut ray, &scene, config.max_bounces)
                })
                .sum::<Color>() / (config.samples as NumericType);


            // Clamp then convert our f64 pixel to the image buffer's u8 version
            let rgb = Rgb([
                ((pixel[0].sqrt()).clamp(0.0, 1.0) * 255.0) as u8,
                ((pixel[1].sqrt()).clamp(0.0, 1.0) * 255.0) as u8,
                ((pixel[2].sqrt()).clamp(0.0, 1.0) * 255.0) as u8,
            ]);
            output.put_pixel(col, height - row - 1, rgb);
        }
    }
}

fn main() {

    let width = 1280;
    let height = 720;
    let options = RenderOptions { samples: 1000, max_bounces: 100 };

    let camera = Camera::new(

        Vector::new(-0.5, 0.0, 3.0),
        Vector::new(1.2, 0.3, -4.0),    // Focal point
        Direction::new(0.0, 1.0, 0.0),
        45.0,
        (width / height) as f64,
        1.0 / 12.0,
    );

    let red_color = Color::new(0.7, 0.3, 0.3);
    let yellow_color = Color::new(0.8, 0.6, 0.1);
    let white_color = Color::new(1.0, 1.0, 1.0);
    let green_color = Color::new(0.6, 0.8, 0.2);

    let red_diffuse_material = Material::Diffuse(Diffuse { color: red_color });
    let blue_diffuse_material = Material::Diffuse(Diffuse { color: Color::new(0.3, 0.3, 0.8) });
    let blue_metallic_material = Material::Metal(Metal { albedo: Color::new(0.8, 0.8, 0.95), fuzz: 0.7 });
    let yellow_metallic_material = Material::Metal(Metal { albedo: Color::new(0.8, 0.6, 0.2), fuzz: 0.2 });
    let dielectric_material = Material::Dielectric(Dielectric { albedo: white_color, refraction_index: 1.5 });
    let yellow_dielectric_material = Material::Dielectric(Dielectric { albedo: yellow_color, refraction_index: 1.1 });


    let objects: Scene = vec![
        ObjectTypes::new_sphere( Point::new(0.0, -601.0, -1.0), 600.0, Material::Diffuse(Diffuse { color: green_color })),
        ObjectTypes::new_sphere( Point::new(-1.4, -0.5, -3.0), 1.0,  blue_diffuse_material.clone()),

        // Hollow Bubble
        ObjectTypes::new_sphere( Point::new(0.6, -0.3, -2.0), 0.5,  dielectric_material.clone() ),
        ObjectTypes::new_sphere( Point::new(0.6, -0.3, -2.0), -0.495, dielectric_material.clone() ),

        ObjectTypes::new_sphere( Point::new(1.8, 0.0, -1.6), 0.2,  yellow_dielectric_material.clone() ),
        ObjectTypes::new_sphere(Point::new(1.5, 0.0, -4.0), 1.5, yellow_metallic_material.clone() ),
        ObjectTypes::new_sphere( Point::new(2.5, -1.0, -2.5), 0.5,  red_diffuse_material.clone() ),
        ObjectTypes::new_sphere(Point::new(2.8, 0.0, -2.0), 0.5, blue_metallic_material.clone() ),

        ObjectTypes::new_triangle(Point::new(-0.4, 0.0, -3.0), Point::new(-0.4, 1.0, -3.0), Point::new(0.3, 0.3, -2.0), blue_diffuse_material.clone()),
        ObjectTypes::new_triangle(Point::new(-0.4, 1.0, -3.0), Point::new(0.6, 1.0, -3.5), Point::new(0.3, 0.3, -2.0), red_diffuse_material.clone()),
    ];

    let mut img = RgbImage::new(width, height);

    println!("Raytracing");
    raytrace(camera, &objects, options, &mut img);
    println!("Saving image to file");
    img.save("samples/output.png");
}
