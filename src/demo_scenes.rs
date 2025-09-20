//! Factory struct that provides predefined rustycs scenes for demo purposes.
use std::mem;

use macroquad::color::{Color, BLACK, WHITE};
use rand::{thread_rng, Rng};
use rustycs::{
    entities::{
        attractor::*,
        body::Body,
        material::{self, Material},
    },
    environment::{force::Force, world::World},
};

use crate::rustycs_utility::{self as util, *};

pub struct WorldScene {
    pub world: World,
    pub bg_color: Color,
    pub spawners: Vec<BodySpawner>,
}

impl WorldScene {
    pub fn new(content: (World, Color, Vec<BodySpawner>)) -> Self {
        Self {
            world: content.0,
            bg_color: content.1,
            spawners: content.2,
        }
    }

    pub fn extract(&mut self) -> (World, Color, Vec<BodySpawner>) {
        (
            mem::take(&mut self.world),
            mem::take(&mut self.bg_color),
            mem::take(&mut self.spawners),
        )
    }
}

pub struct WorldFactory {
    tick_rate: f32,
    default_force: Force,
}

impl WorldFactory {
    pub fn new(tick_rate: f32, default_force: Force) -> WorldFactory {
        Self {
            tick_rate,
            default_force,
        }
    }
}

// demo
#[allow(dead_code)]
impl WorldFactory {
    /// Showcases attractor functionality via a very crude solar system implementation
    pub fn demo_solar_system(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 7.5);
        let (sun, planets) = util::solar_bodies();

        w.add_attractor(sun);
        w.add_bodies(planets);

        WorldScene::new((w, BLACK, vec![]))
    }

    /// Fills up a bounded platform with 100 circles, where the material of the platform and circles can be chosen.
    pub fn demo_filling_pool_with_circles(&self, material: Material) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        w.add_bodies(vec![
            Body::platform_rectangle_obb(0.0, -3.0, 8.0, 0.5, 0., material.clone()),
            Body::platform_rectangle_obb(-4.05, -0.5, 0.1, 5., 0., material.clone()),
            Body::platform_rectangle_obb(4.05, -0.5, 0.1, 5., 0., material.clone()),
        ]);

        let pool_filler = BodySpawner::new_single_type(
            Body::circle(0.0, 10.0, 0.1, material.clone()),
            100,
            30.,
            0.5,
        );

        WorldScene::new((w, WHITE, vec![pool_filler]))
    }

    /// Fills up a bounded platform with 150 random bodies, with random materials and sizes.
    pub fn demo_piling_all_rotating_body_types(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 40.);
        w.add_force(self.default_force.clone());

        let mut pipeline: Vec<Body> = Vec::new();
        let mut rng = thread_rng();

        let mut x: f32;
        let y: f32 = 10.;

        w.add_bodies(vec![
            Body::platform_rectangle_aabb(0.0, -3.0, 24.0, 1.0, material::DEFAULT),
            Body::platform_rectangle_aabb(-12.15, 0.0, 0.1, 10.0, material::DEFAULT),
            Body::platform_rectangle_aabb(12.15, 0.0, 0.1, 10.0, material::DEFAULT),
        ]);

        for _ in 0..100 {
            x = rng.gen_range(-10.0..=10.0);
            pipeline.push(random_body(x, y, &mut rng))
        }

        let body_spawner = BodySpawner::new_pipeline(pipeline, 10.);

        WorldScene::new((w, WHITE, vec![body_spawner]))
    }

    /// Shows circles of all 5 types of materials bouncing on a platform made of a material of choice.
    pub fn demo_material_differences(&self, material: Material) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        let all_circle_materials = vec![
            Body::circle(-4.0, 5.0, 0.2, material::RUBBER),
            Body::circle(-2.0, 5.0, 0.2, material::PLASTIC),
            Body::circle(0.0, 5.0, 0.2, material::STONE),
            Body::circle(2.0, 5.0, 0.2, material::METAL),
            Body::circle(4.0, 5.0, 0.2, material::DEFAULT),
        ];

        w.add_body(Body::platform_rectangle_obb(
            0.0, -3.0, 10.0, 0.5, 0., material,
        ));

        w.add_bodies(all_circle_materials);

        WorldScene::new((w, WHITE, vec![]))
    }

    /// A classic scene regarding physics simulations.<br>
    /// 2 inclined platforms with a level platform at the bottom.<br>
    /// Spawn entities to your liking.
    pub fn demo_slide_and_fall(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        w.add_bodies(vec![
            Body::platform_rectangle_obb(0.0, -4.0, 20.0, 0.5, 0.0, material::DEFAULT),
            Body::platform_rectangle_obb(-3.15, 2.0, 6.0, 0.5, -0.2, material::DEFAULT),
            Body::platform_rectangle_obb(3.15, -1.0, 6.0, 0.5, 0.2, material::DEFAULT),
        ]);

        WorldScene::new((w, WHITE, vec![]))
    }

    /// A simple inclined platform, spawn entities to your liking.
    pub fn demo_slope(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        w.add_body(Body::platform_rectangle_obb(
            0.0,
            -2.0,
            16.0,
            0.2,
            0.3,
            material::DEFAULT,
        ));

        WorldScene::new((w, WHITE, vec![]))
    }

    /// All platform types to observe interactions, spawn entities to your liking.
    pub fn demo_all_platforms(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        w.add_bodies(vec![
            Body::platform_rectangle_obb(-3.0, 2.0, 3.0, 1.0, -0.2, material::DEFAULT),
            Body::platform_circle(3., 0., 1.0, material::DEFAULT),
            Body::platform_rectangle_aabb(0.0, 0.0, 3.0, 1.0, material::DEFAULT),
        ]);

        WorldScene::new((w, WHITE, vec![]))
    }
}

// testing
#[allow(dead_code, unused_variables)]
impl WorldFactory {
    pub fn test_empty(&self) -> WorldScene {
        WorldScene::new((World::new(self.tick_rate, 100.), WHITE, vec![]))
    }

    pub fn test_platform_aabb(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        w.add_bodies(vec![
            Body::platform_rectangle_aabb(0.0, -3.0, 6.0, 1.0, material::DEFAULT),
            Body::platform_rectangle_aabb(-3.15, -3.0, 0.1, 10.0, material::DEFAULT),
            Body::platform_rectangle_aabb(3.15, -3.0, 0.1, 10.0, material::DEFAULT),
        ]);

        WorldScene::new((w, WHITE, vec![]))
    }

    pub fn test_platform_obb(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);
        w.add_force(self.default_force.clone());

        w.add_bodies(vec![
            Body::platform_rectangle_obb(0.0, -3.0, 6.0, 1.0, 0.0, material::DEFAULT),
            Body::platform_rectangle_obb(-3.15, -3.0, 0.1, 10.0, 0.0, material::DEFAULT),
            Body::platform_rectangle_obb(3.15, -3.0, 0.1, 10.0, 0.0, material::DEFAULT),
        ]);

        WorldScene::new((w, WHITE, vec![]))
    }

    pub fn test_local_attractor(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);

        w.add_attractor(Attractor::new(0., 0., 2., AttractorType::Local, None));

        WorldScene::new((w, WHITE, vec![]))
    }

    pub fn test_polygon(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);

        if let Some(poly) = Body::polygon(0.0, 0.0, util::poly_simple(5.0), material::DEFAULT) {
            w.add_body(poly);
        };

        WorldScene::new((w, WHITE, vec![]))
    }

    // should crash -> no concave polygons allowed
    pub fn test_concave_polygon(&self) -> WorldScene {
        let mut w = World::new(self.tick_rate, 100.);

        if let Some(poly) = Body::polygon(0.0, 0.0, util::poly_deep_concave(7.0), material::DEFAULT)
        {
            w.add_body(poly);
        };

        WorldScene::new((w, WHITE, vec![]))
    }
}
