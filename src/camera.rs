use std::f64::consts::PI;
use crate::primitives::{Direction, Vector, Point, NumericType};
use crate::ray::Ray;
use crate::utils::random_point_in_unit_disk;

/// The camera is our render viewport
///
/// Internally we track w,u,v which form the orthonormal basis of the
/// camera's coordinate system, where:
/// - v points along the camera's vertical axis
/// - u points along the camera's horizontal axis
/// - w point at the viewer (not the scene).
///
#[derive(Debug)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,

    horizontal: Vector,
    vertical: Vector,

    aperture: NumericType,

    v: Direction,
    u: Direction,
    w: Direction,
}

impl Camera {
    pub fn new(lookfrom: Vector, lookat: Vector, vup: Direction, vfov: f64, aspect: f64, aperture: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let focus_distance = (lookfrom - lookat).length();

        let lower_left_corner = origin - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w;
        let horizontal = half_width * 2.0 * focus_distance * u;
        let vertical = half_height * 2.0 * focus_distance * v;


        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aperture,
            v,
            u,
            w,
        }
    }

    pub fn get_ray(&self, s: NumericType, t: NumericType) -> Ray {
        let lens_radius = self.aperture / 2.0;
        let rd = lens_radius * random_point_in_unit_disk();

        let offset = self.u * rd[0] + self.v * rd[1];

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset
        )
    }
}