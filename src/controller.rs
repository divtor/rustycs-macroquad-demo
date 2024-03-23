//! A controller struct designed to interact with a rustycs-based
//! world using the game engine "macroquad" for rendering.

// std
use std::time::Instant;

// external
use rand::Rng;

use macroquad::{
    input::KeyCode,
    prelude::{is_key_down, mouse_position, screen_height, screen_width},
};

// my own
use rustycs::{
    attractor::{Attractor, AttractorType::*},
    body::Body,
    material::{self},
    maths::vector2::Vector2,
    world::World,
};

use crate::{
    rustycs_utility as util,
    sim_tracker::{SimulationMode::*, SimulationState},
};

pub const MOVE_CAMERA_UP: KeyCode = KeyCode::W;
pub const MOVE_CAMERA_DOWN: KeyCode = KeyCode::S;
pub const MOVE_CAMERA_LEFT: KeyCode = KeyCode::A;
pub const MOVE_CAMERA_RIGHT: KeyCode = KeyCode::D;
pub const ZOOM_CAMERA_IN: KeyCode = KeyCode::Up;
pub const ZOOM_CAMERA_OUT: KeyCode = KeyCode::Down;
pub const RESET_CAMERA_POS: KeyCode = KeyCode::R;
pub const SPAWN_CIRCLE: KeyCode = KeyCode::Key1;
pub const SPAWN_AABB: KeyCode = KeyCode::Key2;
pub const SPAWN_OBB: KeyCode = KeyCode::Key3;
pub const SPAWN_POLYGON: KeyCode = KeyCode::Key4;
pub const SPAWN_ATTRACTOR: KeyCode = KeyCode::Key5;
pub const TOGGLE_TEXT: KeyCode = KeyCode::T;
pub const OPEN_MENU_AND_PAUSE: KeyCode = KeyCode::Escape;
pub const TOGGLE_HITBOXES: KeyCode = KeyCode::H;
pub const TOGGLE_COLLISION_POINTS: KeyCode = KeyCode::C;
pub const TOGGLE_GRID: KeyCode = KeyCode::G;
pub const WORLD_UPDATE: KeyCode = KeyCode::U;

pub struct UserController {
    pub user_actions: Vec<KeyCode>,
    pub active_actions: Vec<KeyCode>,
    pub scroll_speed: f32,
    pub zoom_speed: f32,
    pub sampling_rate: f32,
    pub sampling_instant: Instant,
}

impl UserController {
    pub fn new(scroll_speed: f32, zoom_speed: f32) -> UserController {
        let user_actions: Vec<KeyCode> = vec![
            MOVE_CAMERA_UP,
            MOVE_CAMERA_DOWN,
            MOVE_CAMERA_LEFT,
            MOVE_CAMERA_RIGHT,
            ZOOM_CAMERA_IN,
            ZOOM_CAMERA_OUT,
            SPAWN_CIRCLE,
            SPAWN_AABB,
            SPAWN_OBB,
            SPAWN_POLYGON,
            SPAWN_ATTRACTOR,
            TOGGLE_TEXT,
            TOGGLE_GRID,
            TOGGLE_HITBOXES,
            TOGGLE_COLLISION_POINTS,
            WORLD_UPDATE,
            RESET_CAMERA_POS,
        ];

        UserController {
            user_actions,
            active_actions: Vec::new(),
            scroll_speed,
            zoom_speed,
            sampling_rate: 1. / 120.,
            sampling_instant: Instant::now(),
        }
    }
}

impl UserController {
    pub fn detect_current_actions(&mut self) {
        self.active_actions.clear();

        if self.sampling_instant.elapsed().as_secs_f32() >= self.sampling_rate {
            for key_code in &self.user_actions {
                if is_key_down(*key_code) {
                    self.active_actions.push(*key_code);
                }
            }
            self.sampling_instant = Instant::now();
        }
    }

    pub fn user_paused(&self) -> bool {
        is_key_down(OPEN_MENU_AND_PAUSE)
    }

    pub fn handle_current_actions(
        &mut self,
        world: &mut World,
        offset_x: &mut f32,
        offset_y: &mut f32,
        state: &mut SimulationState,
    ) {
        if self.active_actions.is_empty() {
            return;
        }

        let mouse_position = mouse_position();
        let world_position = world.screen_to_world(
            mouse_position.0 - *offset_x,
            mouse_position.1 - *offset_y,
            screen_width(),
            screen_height(),
        );

        let mut added: bool = false;

        for action in &self.active_actions {
            match *action {
                SPAWN_AABB => {
                    spawn_aabb(world, world_position);
                    added = true;
                }
                SPAWN_OBB => {
                    spawn_obb(world, world_position);
                    added = true;
                }
                SPAWN_CIRCLE => {
                    spawn_circle(world, world_position);
                    added = true;
                }
                SPAWN_POLYGON => {
                    spawn_polygon(world, world_position);
                    added = true;
                }
                SPAWN_ATTRACTOR => {
                    spawn_attractor(world, world_position);
                    added = true;
                }
                ZOOM_CAMERA_OUT => world.change_ptm_ratio(1. - self.zoom_speed),
                ZOOM_CAMERA_IN => world.change_ptm_ratio(1. + self.zoom_speed),
                MOVE_CAMERA_LEFT => *offset_x += self.scroll_speed,
                MOVE_CAMERA_UP => *offset_y += self.scroll_speed,
                MOVE_CAMERA_RIGHT => *offset_x -= self.scroll_speed,
                MOVE_CAMERA_DOWN => *offset_y -= self.scroll_speed,
                RESET_CAMERA_POS => {
                    *offset_x = 0.;
                    *offset_y = 0.;
                }
                WORLD_UPDATE => {
                    if state.simulation == Paused && state.atomic_update_allowed() {
                        world.update();
                        state.nr_of_updates += 1;
                        state.update_instant = Instant::now();
                    }
                }
                any_toggle => {
                    if state.debug_toggable() {
                        let mut toggled = true;

                        match any_toggle {
                            TOGGLE_TEXT => state.debug_information.toggle(),
                            TOGGLE_GRID => state.debug_grid.toggle(),
                            TOGGLE_COLLISION_POINTS => state.collision_points.toggle(),
                            TOGGLE_HITBOXES => state.hitboxes.toggle(),
                            _ => toggled = false,
                        }

                        if toggled {
                            state.debug_instant = Instant::now();
                        }
                    }
                }
            }
        }

        if added {
            state.spawn_instant = Instant::now();
        }
    }
}

fn spawn_circle(w: &mut World, world_position: Vector2) {
    let mut rng = rand::thread_rng();
    let mat_id: u8 = rng.gen_range(0..4);

    let material = match mat_id {
        0 => material::PLASTIC,
        1 => material::RUBBER,
        2 => material::STONE,
        _ => material::METAL,
    };

    w.add_body(Body::circle(
        world_position.x,
        world_position.y,
        0.2,
        material,
    ))
}

fn spawn_aabb(w: &mut World, world_position: Vector2) {
    w.add_body(Body::aabb(
        world_position.x,
        world_position.y,
        1.0,
        1.0,
        material::DEFAULT,
    ));
}

fn spawn_obb(w: &mut World, world_position: Vector2) {
    w.add_body(Body::obb(
        world_position.x,
        world_position.y,
        1.0,
        1.0,
        material::DEFAULT,
    ));
}

fn spawn_polygon(w: &mut World, world_position: Vector2) {
    if let Some(poly) = Body::polygon(
        world_position.x,
        world_position.y,
        util::poly_complex(3.0),
        material::DEFAULT,
    ) {
        w.add_body(poly);
    }
}

fn spawn_attractor(w: &mut World, world_position: Vector2) {
    let attractor = Attractor::new(world_position.x, world_position.y, 0.0, Global, None);

    w.add_attractor(attractor)
}
