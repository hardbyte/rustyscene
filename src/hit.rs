use std::ops::Range;
use crate::material::{Material, Scatter};
use crate::primitives::{Direction, Point, Time};
use crate::ray::Ray;

pub struct HitRecord<'a> {
    pub t: Time,
    pub point: Point,
    pub normal: Direction,
    pub material: &'a Material,

}

pub trait Hittable<'a>: Sync + Send {
    fn hits(&self, ray: &Ray, interval: &Range<Time>) -> Option<HitRecord>;
}


