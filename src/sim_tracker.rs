//! A simulation state tracking struct designed to interact with a rustycs-based
//! world using the game engine "macroquad" for rendering.

use std::time::Instant;

#[derive(PartialEq)]
pub enum ShowDebug {
    Visible,
    Hidden,
}

impl Default for ShowDebug {
    fn default() -> Self {
        Self::Hidden
    }
}

impl ShowDebug {
    pub fn toggle(&mut self) {
        use ShowDebug::*;

        match self {
            Visible => *self = Hidden,
            Hidden => *self = Visible,
        }
    }
}

#[derive(PartialEq)]
pub enum SimulationMode {
    Running,
    Paused,
}

impl Default for SimulationMode {
    fn default() -> Self {
        Self::Running
    }
}

impl SimulationMode {
    pub fn toggle(&mut self) {
        use SimulationMode::*;

        match self {
            Running => *self = Paused,
            Paused => *self = Running,
        }
    }
}

pub struct SimulationState {
    pub debug_information: ShowDebug,
    pub collision_points: ShowDebug,
    pub hitboxes: ShowDebug,
    pub debug_grid: ShowDebug,
    pub grid_ratio: f32,
    pub simulation: SimulationMode,
    pub debug_instant: Instant,
    pub debug_timeout: f32,
    pub spawn_instant: Instant,
    pub spawn_timeout: f32,
    pub tick_instant: Instant,
    pub tick_timeout: f32,
    pub pause_instant: Instant,
    pub pause_timeout: f32,
    pub update_instant: Instant,
    pub update_timeout: f32,
    pub nr_of_updates: u32,
    pub max_update_duration: f32,
}

impl SimulationState {
    pub fn new(tick_timeout: f32) -> Self {
        Self {
            tick_timeout,
            ..Default::default()
        }
    }
}

impl SimulationState {
    pub fn is_pausable(&self) -> bool {
        self.pause_instant.elapsed().as_secs_f32() >= self.pause_timeout
    }

    pub fn is_spawnable(&self) -> bool {
        self.spawn_instant.elapsed().as_secs_f32() >= self.spawn_timeout
    }

    pub fn debug_toggable(&mut self) -> bool {
        self.debug_instant.elapsed().as_secs_f32() >= self.debug_timeout
    }

    pub fn update_required(&self) -> bool {
        self.tick_instant.elapsed().as_secs_f32() >= self.tick_timeout
    }

    pub fn atomic_update_allowed(&self) -> bool {
        self.update_instant.elapsed().as_secs_f32() >= self.update_timeout
    }
}

impl Default for SimulationState {
    fn default() -> Self {
        use ShowDebug::*;
        use SimulationMode::*;

        Self {
            debug_information: Visible,
            collision_points: Hidden,
            hitboxes: Hidden,
            debug_grid: Hidden,
            debug_instant: Instant::now(),
            debug_timeout: 0.25,
            grid_ratio: 10.,
            simulation: Running,
            spawn_instant: Instant::now(),
            spawn_timeout: 0.15,
            tick_instant: Instant::now(),
            tick_timeout: 1. / 64.,
            pause_instant: Instant::now(),
            pause_timeout: 0.25,
            update_instant: Instant::now(),
            update_timeout: 0.25,
            nr_of_updates: 0,
            max_update_duration: 0.,
        }
    }
}
