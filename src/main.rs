extern crate nalgebra;
use nalgebra::{Vector3, norm, dot};

fn acceleration(r: &Vector3<f64>) -> Vector3<f64> {
    let weigth = Vector3::new(0.0, 0.0, -9.8);
    let tension = - dot(&weigth, r) / norm(r).powi(2) * r;
    tension + weigth
}

fn main() {
    let r = Vector3::new(0.1, 0.0, -1.0);
    let a = acceleration(&r);
    println!("Acceleration {}", a);
}
