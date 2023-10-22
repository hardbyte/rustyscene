use crate::hit::HitRecord;
use crate::material::{Scatter, Scattered};
use crate::primitives::Color;
use crate::ray::Ray;
use crate::utils::{random_point_in_unit_sphere, random_point_on_hemisphere};

#[derive(Clone)]
pub struct Diffuse {
    pub color: Color
}

impl Scatter for Diffuse {
    fn scatter(&self, ray: &mut Ray, hit_record: &HitRecord) -> Scattered {
        ray.origin = hit_record.point;
        ray.direction = hit_record.point + hit_record.normal + random_point_in_unit_sphere().normalize();

        Scattered { scattered: true, attenuation: self.color }
    }
}