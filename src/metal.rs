use crate::hit::HitRecord;
use crate::material::{Scatter, Scattered};
use crate::primitives::Color;
use crate::ray::Ray;
use crate::utils::{random_point_in_unit_sphere, random_point_on_hemisphere, reflect};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Scatter for Metal {
    fn scatter(&self, ray: &mut Ray, hit_record: &HitRecord) -> Scattered {
        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        let black = Color::new(0.0, 0.0, 0.0);

        ray.origin = hit_record.point;
        ray.direction = reflected + self.fuzz * random_point_in_unit_sphere();

        let is_scattered = reflected.dot(hit_record.normal) > 0.0;

        if is_scattered {
            Scattered { scattered: true, attenuation: self.albedo }
        } else {

            Scattered { scattered: false, attenuation: black }
        }

    }
}