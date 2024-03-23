use std::time::Instant;

use macroquad::{
    prelude::{next_frame, Conf},
    window::{screen_height, screen_width},
};

#[allow(unused_imports)]
use rustycs::{
    force::{Force, GRAVITY_EARTH},
    material,
};

use rustycs_macroquad_demo::{
    sim_tracker::{ShowDebug::*, SimulationMode::*},
    *,
};

#[allow(dead_code)]
/// Source: https://www.khronos.org/opengl/wiki/Swap_Interva <br>
/// Note: Only tested on windows.
enum Vsync {
    Deactivated = 0,
    Activated = 1,
    Adaptive = -1,
}

const VSYNC_SETTING: Option<i32> = Some(Vsync::Deactivated as i32);

fn window_config() -> Conf {
    let mut conf = Conf {
        window_resizable: false,
        window_width: WINDOW_SIZE.0,
        window_height: WINDOW_SIZE.1,
        ..Default::default()
    };

    conf.platform.swap_interval = VSYNC_SETTING;
    conf
}

const WINDOW_SIZE: (i32, i32) = (1000, 1000);
const TICK_RATE: f32 = 256.;
const FORCE: Force = GRAVITY_EARTH;

#[macroquad::main(window_config)]
async fn main() {
    // ------------------------------ SETUP ------------------------------
    let factory = WorldFactory::new(TICK_RATE, FORCE);

    // choose available scene from factory
    let mut scene = factory.demo_all_platforms();

    let (mut world, bg_color, mut spawners) = scene.extract();
    world.set_collision_precision(25);

    let mut state: SimulationState = SimulationState::new(world.get_delta_time());
    let mut controller: UserController = UserController::new(10.0, 0.01);

    let (w, h) = (screen_width(), screen_height());
    let (mut offset_x, mut offset_y): (f32, f32) = (0.0, 0.0);

    // ------------------------------ SIMULATION LOOP ------------------------------
    loop {
        if state.update_required() && state.simulation == Running {
            world.update();
            state.nr_of_updates += 1;
            state.tick_instant = Instant::now();
        }

        if state.simulation == Running {
            for spawner in &mut spawners {
                if spawner.is_spawnable() {
                    world.add_body(spawner.spawn());
                    spawner.timer = Instant::now();
                }
            }
        }

        if state.is_pausable() && controller.user_paused() {
            state.simulation.toggle();
            state.pause_instant = Instant::now();
        }

        controller.detect_current_actions();

        if !controller.active_actions.is_empty() && state.is_spawnable() {
            controller.handle_current_actions(&mut world, &mut offset_x, &mut offset_y, &mut state);
        }

        render_world(&world, offset_x, offset_y, &state, bg_color);

        if state.debug_information == Visible {
            let cam_x = w * 0.5 - offset_x;
            let cam_y = w * 0.5 - offset_y;
            let cam_loc = world.screen_to_world(cam_x, cam_y, w, h);

            render_info_and_benchmark(
                &mut state,
                world.get_bodies().len(),
                world.get_last_update_duration(),
                cam_loc,
            );
        }

        next_frame().await
    }
}
