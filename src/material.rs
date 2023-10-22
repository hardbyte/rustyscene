use crate::dielectric::Dielectric;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::primitives::Vector;
use crate::diffuse::Diffuse;
use crate::metal::Metal;

#[derive(Clone)]
pub enum Material {

    Diffuse(Diffuse),

    Metal(Metal),
    Dielectric(Dielectric),

    //Lambertian { albedo: Texture },
    //DiffuseLight(Texture),
    //Isotropic { albedo: Texture },
}

pub struct Scattered {
    pub attenuation: Vector,
    pub scattered: bool,
}

pub trait Scatter {
    fn scatter(
        &self,
        ray_in: &mut Ray,
        hit_record: &HitRecord
    ) -> Scattered;
}

impl Scatter for Material {
    fn scatter(&self, ray_in: &mut Ray, hit_record: &HitRecord) -> Scattered {
        match self {
            Material::Diffuse(diffuse) => diffuse.scatter(ray_in, hit_record),
            Material::Metal(metal) => metal.scatter(ray_in, hit_record),
            Material::Dielectric(dielectric) => dielectric.scatter(ray_in, hit_record),
        }
    }
}