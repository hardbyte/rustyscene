use std::ops::Range;
use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::primitives::{Direction, Point, Time};
use crate::ray::Ray;

pub struct Triangle {

    p1: Point,
    p2: Point,
    p3: Point,

    normal: Direction,
    material: Material
}


impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point, material: Material) -> Self {
        let v1 = p2 - p1;
        let v2 = p3 - p1;
        let normal = v1.cross(v2).normalize();


        Triangle {
            p1,
            p2,
            p3,
            normal,
            material,
        }
    }
    pub fn p1(&self) -> &Point {
        &self.p1
    }

    pub fn p2(&self) -> &Point {
        &self.p2
    }

    pub fn p3(&self) -> &Point {
        &self.p3
    }

    pub fn normal(&self) -> &Direction {
        &self.normal
    }

}

impl Hittable<'_> for Triangle {
    fn hits(&self, ray: &Ray, interval: &Range<Time>) -> Option<HitRecord> {
        let edge1 = self.p2 - self.p1;
        let edge2 = self.p3 - self.p1;

        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -f64::EPSILON && a < f64::EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.p1;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);

        if t > interval.start && t < interval.end {
            let point = ray.at(t);
            let normal = self.normal.normalize();
            return Some(HitRecord {
                t,
                point,
                normal,
                material: &self.material,
            });
        }

        None
    }
}
