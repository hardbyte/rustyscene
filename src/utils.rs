use rand::Rng;
use crate::primitives::{Direction, NumericType, Point, Vector};

pub fn random_point_in_unit_disk() -> Point {

    const ONES: Point = Point::new(1.0, 1.0, 1.0);

    let point_in_unit_square = || {
        let p: Point = Point::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());
        2.0 * p - ONES
    };

    let mut p = point_in_unit_square();
    while (p.dot(p) >= 1.0) {
        p = point_in_unit_square();
    }
    p[2] = 0.0;
    return p;
}


pub fn random_point_in_unit_sphere() -> Point {
    let mut rng = rand::thread_rng();
    loop {
        let vec = Point::new(
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
            rng.gen_range(-1.0..1.),
        );

        if vec.length_squared() < 1. {
            break vec;
        }
    }
}

pub fn random_point_on_hemisphere(normal: Direction) -> Point {
    let p = random_point_in_unit_sphere().normalize();
    if p.dot(normal) > 0.0 {
        return p;
    } else {
        return -p;
    }
}

pub fn reflect(v: Vector, n: Vector) -> Vector {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Direction, n:Direction, ni_over_nt: NumericType) -> Option<Direction> {
    let uv = v.normalize();
    let dt = uv.dot(n);

    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt.powi(2));

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt ) - ( n * discriminant.sqrt());
        return Some(refracted);
    } else {
        return None
        // return Some(Vector::new(0.0, 0.0, 0.0));
    }

}

