extern crate nalgebra;
use nalgebra::{Vector3, norm, dot};

type Vector = Vector3<f64>;

fn acceleration(r: &Vector) -> Vector {
    let weigth = Vector::new(0.0, 0.0, -9.8);
    let tension = -dot(&weigth, r) / norm(r).powi(2) * r;
    tension + weigth
}

fn verlet_step(r: &Vector, r_old: &Vector, a: &Fn(&Vector) -> Vector, delta_t: f64) -> Vector {
    2.0 * r - r_old + delta_t.powi(2) * a(r)
}

fn verlet_integration(r: &Vector,
                      v: &Vector,
                      a: &Fn(&Vector) -> Vector,
                      time: f64,
                      delta_t: f64)
                      -> Vec<Vector> {
    let mut r_old = r.clone();
    let mut r = r_old + v * delta_t + 0.5 * a(r) * delta_t.powi(2);
    let mut t = 0.0;

    let mut res = Vec::new();
    res.push(r_old);
    while t < time {
        t = t + delta_t;
        res.push(r);
        let temp = verlet_step(&r, &r_old, a, delta_t);
        r_old = r;
        r = temp;
    }
    res
}

fn main() {
    let r = Vector3::new(0.1, 0.0, -1.0);
    let trajectory = verlet_integration(
        &r,
        &Vector::new(0.0, 0.1, 0.0),
        &acceleration,
        5.0,
        1e-6
    );

    for item in trajectory {
        println!("{} {} {}", item.x, item.y, item.z);
    }
}
