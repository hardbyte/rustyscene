use rand::random;
use crate::hit::HitRecord;
use crate::material::{Scatter, Scattered};
use crate::primitives::{Color, Direction, NumericType};
use crate::ray::Ray;
use crate::utils::{reflect, refract};

#[derive(Clone)]
pub struct Dielectric {
    pub albedo: Color,

    pub refraction_index: NumericType,
    // specular_exponent: f64,
}

fn schlick(cosine: NumericType, refraction_index: NumericType) -> NumericType {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &mut Ray, hit_record: &HitRecord) -> Scattered {

        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        // The Dielectric material can tint the light refracted through it.
        let attenuation = self.albedo;

        let is_scattered = ray.direction.dot(hit_record.normal) > 0.0;

        let mut outward_normal: Direction;
        let ni_over_nt: NumericType;
        let cosine: NumericType;

        if is_scattered {
            outward_normal = - hit_record.normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * ray.direction.dot(hit_record.normal) / (ray.direction).length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -ray.direction.dot(hit_record.normal) / (ray.direction).length();
        }

        let refracted = refract(ray.direction, outward_normal, ni_over_nt);

        let reflect_probability = match refracted {
            None => 1.0,
            Some(_) => schlick(cosine, self.refraction_index)
        };

        let scattered_direction = if random::<NumericType>() < reflect_probability {
            reflected
        } else {
            refracted.unwrap()
        };

        // Mutate the ray
        ray.origin = hit_record.point;
        ray.direction = scattered_direction;
        Scattered { attenuation, scattered: true }
    }
}