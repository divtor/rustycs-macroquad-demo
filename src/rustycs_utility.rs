//! Utility functions used to improve interactions with the rustycs library.<br>
//! Mainly content that is necessary for ease of use e.g. within scenes, but has no place in the engine itself.<br><br>
//! For example, it provides functionality such as:
//! - a line struct to enable easier line rendering
//! - predefined polygon shapes
//! - random body functionality
//! - a body spawner
//! - demo scene specific definitions (e.g. solar system)

#![allow(unused_variables, dead_code)]
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{fmt::Display, ops, time::Instant};

use rustycs::{
    attractor::{Attractor, AttractorType::*},
    body::{Body, BodyType::*},
    material,
    maths::Vector2,
    shapes::{Circle, Polygon, Shape, AABB},
};

// ------------------- Lines -------------------
#[derive(Clone, Debug)]
pub struct Line {
    pub from_x: f32,
    pub from_y: f32,
    pub to_x: f32,
    pub to_y: f32,
}

impl Line {
    pub fn new(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> Line {
        Line {
            from_x,
            from_y,
            to_x,
            to_y,
        }
    }

    pub fn apply_screen_location(&mut self, x: f32, y: f32) {
        self.from_x += x;
        self.from_y = -self.from_y + y;
        self.to_x += x;
        self.to_y = -self.to_y + y;
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "from ({:.2}/{:.2}) to ({:.2}/{:.2})",
            self.from_x, self.from_y, self.to_x, self.to_y
        )
    }
}

impl ops::MulAssign<f32> for Line {
    fn mul_assign(&mut self, rhs: f32) {
        self.from_x *= rhs;
        self.from_y *= rhs;
        self.to_x *= rhs;
        self.to_y *= rhs;
    }
}

// ------------------- Random Shape -------------------
pub fn random_body(x: f32, y: f32, rng: &mut ThreadRng) -> Body {
    Body::new(x, y, random_shape(rng), Dynamic, random_material(rng), None)
}

fn random_shape(rng: &mut ThreadRng) -> Shape {
    let mut rng = thread_rng();

    let shape: u8 = rng.gen_range(0..3);

    match shape {
        0 => {
            let r: f32 = rng.gen_range(0.1..0.5);
            Shape::Circle(Circle::new(r))
        }
        1 => {
            let width = rng.gen_range(0.2..1.0);
            let height = rng.gen_range(0.2..1.0);

            Shape::Polygon(Polygon::new(AABB::generate_corners(width, height).to_vec()).unwrap())
        }
        _ => {
            let poly_type: u8 = rng.gen_range(0..2);
            let scale: f32 = rng.gen_range(1.0..=3.0);

            match poly_type {
                0 => Shape::Polygon(Polygon::new(poly_simple(scale)).unwrap()),
                1 => Shape::Polygon(Polygon::new(poly_triangle(scale)).unwrap()),
                _ => Shape::Polygon(Polygon::new(poly_complex(scale)).unwrap()),
            }
        }
    }
}

fn random_material(rng: &mut ThreadRng) -> material::Material {
    let material: u8 = rng.gen_range(0..4);

    match material {
        0 => material::RUBBER,
        1 => material::PLASTIC,
        2 => material::STONE,
        _ => material::METAL,
    }
}

// ------------------- Polygon shape constructors -------------------
// DEFINITION IN CLOCKWISE ORDER

pub fn poly_triangle(scale: f32) -> Vec<Vector2> {
    vec![
        Vector2::new(0.0, 0.115_333_33) * scale,
        Vector2::new(0.1, -0.057_666_667) * scale,
        Vector2::new(-0.1, -0.057_666_667) * scale,
    ]
}

pub fn poly_simple(scale: f32) -> Vec<Vector2> {
    vec![
        Vector2::new(0.3, 0.3) * scale,
        Vector2::new(0.5, -0.02) * scale,
        Vector2::new(-0.01, -0.05) * scale,
        Vector2::new(-0.23, 0.04) * scale,
    ]
}

pub fn poly_complex(scale: f32) -> Vec<Vector2> {
    vec![
        Vector2::new(-0.12, -0.308) * scale,
        Vector2::new(0.0, -0.4) * scale,
        Vector2::new(0.4, 0.12) * scale,
        Vector2::new(0.1, 0.35) * scale,
        Vector2::new(-0.13, -0.1) * scale,
    ]
}

// concave testing
pub fn poly_deep_concave(scale: f32) -> Vec<Vector2> {
    vec![
        Vector2::new(-1.0, 1.0) * scale,
        Vector2::new(0.0, 0.1) * scale,
        Vector2::new(1.0, 1.0) * scale,
        Vector2::new(0.1, 0.0),
        Vector2::new(1.0, -1.0) * scale,
        Vector2::new(0.0, -0.1) * scale,
        Vector2::new(-1.0, -1.0) * scale,
        Vector2::new(-0.1, 0.0),
    ]
}

pub fn poly_shallow_concave(scale: f32) -> Vec<Vector2> {
    vec![
        Vector2::new(-1.0, 1.0) * scale,
        Vector2::new(0.0, 1.0) * scale,
        Vector2::new(1.0, 1.0) * scale,
        Vector2::new(1.0, 0.0),
        Vector2::new(1.0, -1.0) * scale,
        Vector2::new(0.0, -1.0) * scale,
        Vector2::new(-1.0, -1.0) * scale,
        Vector2::new(-0.95, 0.0),
    ]
}

// ------------------- Solar scene utility -------------------
pub fn solar_bodies() -> (Attractor, Vec<Body>) {
    // (name, distance to sun, mass, orbital velocity, radius)
    let planet_data: [(&'static str, f32, f32, f32, f32); 8] = [
        ("mercury", 6_f32, 2.1, 18_f32, 0.5),
        ("venus", -12_f32, 3.3, -20_f32, 0.5),
        ("earth", 18_f32, 4.3, 24_f32, 0.5),
        ("mars", -24_f32, 5.1, -30_f32, 0.5),
        ("jupiter", 30_f32, 6.1, 36_f32, 1_f32),
        ("saturn", -36_f32, 7.1, -42_f32, 0.8),
        ("uranus", 42_f32, 8.1, 48_f32, 0.65),
        ("neptune", 48_f32, 9.1, 54_f32, 0.65),
    ];

    let mut planets: Vec<Body> = Vec::new();

    for p in planet_data {
        planets.push(planet(p.0, p.1, 0., p.2, Vector2::new(0.0, p.3), p.4));
    }

    (sun(0., 0.), planets)

    /*
        // data source: https://nssdc.gsfc.nasa.gov/planetary/factsheet/
        // (distance 10^6 km, mass 12^24 kg, name, orbital_velocity km/s, radius km)
        (58., 0.33, "mercury", 47., 4_879. / 2.),
        (108., 4.87, "venus", 35., 12_104. / 2.),
        (149.6, 5.97, "earth", 29.8, 12_756. / 2.),
        (228., 0.642, "mars", 21.1, 6_792. / 2.),
        (778.5, 1898., "jupiter", 13.1, 142_984. / 2.),
        (1432., 568., "saturn", 9.7, 120_536. / 2.),
        (2867., 86.8, "uranus", 6.8, 51_118. / 2.),
        (5900.4, 102., "neptune", 5.4, 49_528. / 2.),
    */
}

pub fn sun(x: f32, y: f32) -> Attractor {
    Attractor::new(x, y, 0.0, Global, Some("sun")).clamp_distance(0.1, 100.)
}

#[allow(unused_variables, unused_mut)]
pub fn planet(
    name: &'static str,
    x: f32,
    y: f32,
    mass: f32,
    velocity: Vector2,
    radius: f32,
) -> Body {
    let mut planet = Body::new(
        x,
        y,
        Shape::Circle(Circle::new(radius)),
        Dynamic,
        material::DEFAULT,
        Some(name),
    );

    planet.set_mass(mass);
    planet.apply_impulse(velocity);

    planet
}

// ------------------- BodySpawner UTILITY -------------------
#[derive(Debug)]
pub enum SpawnerType {
    Single,
    Pipeline,
}

#[derive(Debug)]
pub struct BodySpawner {
    pub body: Body,
    pub body_pipeline: Vec<Body>,
    pub amount: u8,
    pub count: u8,
    pub timer: Instant,
    pub period: f32,
    pub t: SpawnerType,
    pub offset: f32,
}

impl BodySpawner {
    pub fn new_single_type(
        body: Body,
        amount: u8,
        frequency_in_hz: f32,
        offset: f32,
    ) -> BodySpawner {
        BodySpawner {
            body,
            amount,
            period: 1. / frequency_in_hz,
            offset,
            ..Default::default()
        }
    }

    pub fn new_pipeline(body_pipeline: Vec<Body>, frequency_in_hz: f32) -> BodySpawner {
        BodySpawner {
            body_pipeline,
            period: 1. / frequency_in_hz,
            t: SpawnerType::Pipeline,
            ..Default::default()
        }
    }
}

impl BodySpawner {
    pub fn spawn(&mut self) -> Body {
        match self.t {
            SpawnerType::Single => {
                self.count += 1;
                let mut rng = rand::thread_rng();

                let mut offset_x: f32 = 0.0;
                let mut offset_y: f32 = 0.0;

                if self.offset != 0. {
                    offset_x = rng.gen_range(-self.offset..self.offset);
                    offset_y = rng.gen_range(-self.offset..self.offset);
                }

                let mut b = self.body.clone();
                b.transform.location.x += offset_x;
                b.transform.location.y += offset_y;
                b
            }
            SpawnerType::Pipeline => self
                .body_pipeline
                .pop()
                .expect("should be checked in is_spawnable()"),
        }
    }

    pub fn is_spawnable(&mut self) -> bool {
        match self.t {
            SpawnerType::Single => {
                self.count < self.amount && self.timer.elapsed().as_secs_f32() >= self.period
            }
            SpawnerType::Pipeline => {
                !self.body_pipeline.is_empty() && self.timer.elapsed().as_secs_f32() >= self.period
            }
        }
    }
}

impl Default for BodySpawner {
    fn default() -> Self {
        Self {
            body: Default::default(),
            body_pipeline: Default::default(),
            amount: 1,
            count: 0,
            timer: Instant::now(),
            period: 1.,
            t: SpawnerType::Single,
            offset: 0.0,
        }
    }
}
