use std::ops::Range;
use crate::primitives::{Direction, NumericType, Point, Time};
use crate::hit::{Hittable, HitRecord};
use crate::material::Material;
use crate::ray::Ray;


pub struct Sphere {
    pub center: Point,
    pub radius: NumericType,
    pub material: Material
}

impl Hittable<'_> for Sphere {
    fn hits(&self, ray: &Ray, interval: &Range<Time>) -> Option<HitRecord> {
        let t_min = interval.start;
        let t_max = interval.end;

        let oc = ray.origin - self.center;

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius.powi(2);

        let discriminant = b.powi(2) - a * c;
        if discriminant > 0_f64 {
            let t0 = (-b - discriminant.sqrt())/a;
            let t1 = (-b + discriminant.sqrt())/a;

            let mut t = NumericType::MAX;

            if (t0 < t_max) && (t0 > t_min) {
                t = t0;
            } else if (t1 < t_max) && (t1 > t_min) {
                t = t1;
            } else {
                return None
            }
            let p: Point = ray.at(t);
            let n: Direction = (p - self.center)/self.radius;

            return Some(HitRecord {
                t,
                point: p,
                normal: n,
                material: &self.material
            })
        }
        return None

    }
}

