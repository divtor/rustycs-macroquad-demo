//! This renderer is designed to render a rustycs-based physics
//! world using the game engine "macroquad" for rendering.

use std::time::Instant;

use macroquad::{
    color::*,
    prelude::mouse_position,
    shapes::{draw_circle, draw_circle_lines, draw_line},
    text::draw_text,
    time::get_fps,
    window::{clear_background, screen_height, screen_width},
};

use rustycs::{
    attractor::AttractorType::*, body::BodyType::*, collision::Hitbox, maths::Vector2,
    shapes::Shape::*, transforms::Transform, world::World,
};

use crate::{
    rustycs_utility::Line,
    sim_tracker::{ShowDebug::*, SimulationMode::*, SimulationState},
};

const DEBUG_LINE_THICKNESS: f32 = 1.;
const BODY_LINE_THICKNESS: f32 = 2.;
const FONT_SIZE: f32 = 20.;
const UI_TEXT_COLOR: Color = BLACK;
const UI_TEXT_COLOR_PAUSED: Color = WHITE;

pub fn render_world(
    world: &World,
    offset_x: f32,
    offset_y: f32,
    state: &SimulationState,
    bg_color: Color,
) -> f32 {
    let render_start = Instant::now();

    let ratio = world.get_ptm_ratio();
    let w = screen_width();
    let h = screen_height();

    let mp = mouse_position();
    let mouse_hover_pos = world.screen_to_world(mp.0 - offset_x, mp.1 - offset_y, w, h);

    if state.simulation == Paused {
        clear_background(LIGHTGRAY);
    } else {
        clear_background(bg_color);
    }

    let mut body_debug_location: Option<(f32, f32)> = None;
    let mut body_debug_arrow: Option<Line> = None;
    let mut body_debug_info: Option<&Transform> = None;

    if state.debug_grid == Visible {
        render_grid_f(w, h, ratio);
    }

    world.get_bodies().iter().for_each(|body| {
        // screen position of body.transform.location without camera offset
        let (mut x, mut y) = world.world_to_screen(body.transform.location, w, h);

        // absolute screen position of body.transform.location
        (x, y) = (x + offset_x, y + offset_y);

        let material_color = get_material_color(body.material.name);

        if let Some(name) = body.name {
            let r = body.shape.as_circle().r;

            let color = match name {
                "mercury" => BROWN,
                "venus" => WHITE,
                "earth" => BLUE,
                "mars" => RED,
                "jupiter" => ORANGE,
                "saturn" => GOLD,
                "uranus" => SKYBLUE,
                "neptune" => DARKBLUE,
                _ => BLACK,
            };

            draw_circle(x, y, r * ratio, color)
        } else if body.body_type == Static {
            match &body.shape {
                Circle(c) => {
                    draw_circle_lines(x, y, c.r * ratio, BODY_LINE_THICKNESS, material_color);
                }

                AABB(_) => {
                    for line in get_body_outlines(body.get_vertices_as_vec(), ratio, x, y) {
                        render_line(line, material_color);
                    }
                }

                Polygon(_) => {
                    for line in get_body_outlines(body.get_vertices_as_vec(), ratio, x, y) {
                        render_line(line, material_color);
                    }
                }
            }
        } else {
            match &body.shape {
                Circle(c) => {
                    draw_circle(x, y, c.r * ratio, material_color);

                    let vp = &body.vertices[0];
                    draw_line(x, y, x + vp.x * ratio, y - vp.y * ratio, 1.0, WHITE)
                }

                AABB(_) => {
                    for line in get_body_outlines(body.get_vertices_as_vec(), ratio, x, y) {
                        render_line(line, material_color);
                    }
                }

                Polygon(_) => {
                    for line in get_body_outlines(body.get_vertices_as_vec(), ratio, x, y) {
                        render_line(line, material_color);
                    }
                }
            }
        }

        if state.hitboxes == Visible {
            let box_corners = get_hitbox_vertices(&body.hitbox);
            let lines = get_hitbox_outlines(box_corners, ratio, x, y);
            for line in lines {
                render_line(line, BLUE);
            }
        }

        if state.simulation == Paused && body.body_type == Dynamic && body.encloses(mouse_hover_pos)
        {
            let loc = body.transform.location;
            let vel = body.transform.velocity;
            let visual_scale = 20. * world.get_delta_time();
            let vel_vis = loc + vel * visual_scale;

            // screen coords + camera offset
            let (mut x_vel, mut y_vel) = world.world_to_screen(vel_vis, w, h);
            (x_vel, y_vel) = (x_vel + offset_x, y_vel + offset_y);

            body_debug_arrow = Some(Line::new(x, y, x_vel, y_vel));
            body_debug_location = Some((x, y));
            body_debug_info = Some(&body.transform);
        }
    });

    world.get_attractors().iter().for_each(|attractor| {
        let (mut x, mut y) = world.world_to_screen(attractor.location, w, h);
        (x, y) = (x + offset_x, y + offset_y);

        if let Some(name) = attractor.name {
            if name == "sun" {
                draw_circle(x, y, 2. * ratio, YELLOW);
            }
        } else {
            draw_circle(x, y, 2.0, BLACK);
        }

        if state.hitboxes == Visible && attractor.a_type == Local {
            draw_circle_lines(x, y, attractor.r * ratio, DEBUG_LINE_THICKNESS, BLUE)
        }
    });

    if state.collision_points == Visible {
        for p in &world.collision_points {
            let (x, y) = world.world_to_screen(*p, w, h);
            draw_circle(x + offset_x, y + offset_y, 3., BLUE);
        }
    }

    // so nothing gets drawn over debug info
    if state.simulation == Paused {
        if let (Some((x, y)), Some(line), Some(info)) =
            (body_debug_location, body_debug_arrow, body_debug_info)
        {
            render_velocity_pointer(line, WHITE, ratio);
            render_body_info(x, y, info);
        }
    }

    render_start.elapsed().as_secs_f32()
}

// ---------------------- RENDER UTILITY ----------------------
const GRID_RATIO: f32 = 10.;

fn render_grid_f(width: f32, height: f32, _ptm_ratio: f32) {
    let w_ratio = width / GRID_RATIO;
    let h_ratio = height / GRID_RATIO;

    for line_idx in 0..(GRID_RATIO as i32) {
        let x = w_ratio * line_idx as f32;
        let y = h_ratio * line_idx as f32;
        draw_line(x, 0.0, x, height, 1.0, LIGHTGRAY);
        draw_line(0.0, y, width, y, 1.0, LIGHTGRAY);
    }

    draw_circle(width * 0.5, height * 0.5, 2., BLACK);
}

fn render_line(line: Line, color: macroquad::color::Color) {
    draw_line(
        line.from_x,
        line.from_y,
        line.to_x,
        line.to_y,
        BODY_LINE_THICKNESS,
        color,
    );
}

fn render_velocity_pointer(arrow_line: Line, color: macroquad::color::Color, ratio: f32) {
    draw_line(
        arrow_line.from_x,
        arrow_line.from_y,
        arrow_line.to_x,
        arrow_line.to_y,
        BODY_LINE_THICKNESS,
        color,
    );

    draw_circle(arrow_line.to_x, arrow_line.to_y, 0.01 * ratio, color);
}

fn render_body_info(x: f32, y: f32, transform: &Transform) {
    let infos: [&str; 3] = [
        &format!("location: {location}", location = transform.location),
        &format!("velocity: {velocity}", velocity = transform.velocity),
        &format!(
            "angular velocity: {angular_vel:.7}",
            angular_vel = transform.angular_velocity
        ),
    ];

    for (idx, info) in infos.iter().enumerate() {
        draw_text(
            info,
            x + 50.,
            y - 20. + (20. * idx as f32),
            FONT_SIZE,
            UI_TEXT_COLOR_PAUSED,
        )
    }
}

// ---------------------- RENDER GETTERS ----------------------
fn get_material_color(material_type: &'static str) -> Color {
    match material_type {
        // also black to properly see objects
        "rubber" => RED,
        "plastic" => GREEN,
        "stone" => GRAY,
        "metal" => DARKGRAY,
        "default" => BLACK,
        _ => BLACK,
    }
}

fn get_body_outlines(vertices: Vec<Vector2>, ratio: f32, x: f32, y: f32) -> Vec<Line> {
    let nr_vertices = vertices.len();

    if nr_vertices < 3 {
        panic!("Cannot generate lines for shapes with less than 3 vertices.")
    }

    let mut lines: Vec<Line> = Vec::new();

    for idx in 0..nr_vertices {
        let from = &vertices[idx];
        let to = &vertices[(idx + 1) % nr_vertices];

        let mut line = Line::new(from.x, from.y, to.x, to.y);

        line *= ratio;
        line.apply_screen_location(x, y);

        lines.push(line)
    }

    lines
}

fn get_hitbox_vertices(hitbox: &Hitbox) -> Vec<Vector2> {
    vec![
        Vector2::new(hitbox.min.x, hitbox.max.y),
        Vector2::new(hitbox.max.x, hitbox.max.y),
        Vector2::new(hitbox.max.x, hitbox.min.y),
        Vector2::new(hitbox.min.x, hitbox.min.y),
    ]
}

fn get_hitbox_outlines(vertices: Vec<Vector2>, ratio: f32, x: f32, y: f32) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    for idx in 0..4 {
        let from = &vertices[idx];
        let to = &vertices[(idx + 1) % 4];

        let mut line = Line::new(from.x, from.y, to.x, to.y);

        line *= ratio;
        line.apply_screen_location(x, y);

        lines.push(line)
    }

    lines
}

// ---------------------- INFO ----------------------
const PAUSE_MENU_INFO: &str = "Press [ESC] to pause the simulation and show options.";

const MANUAL: [&str; 8] = [
    "[1] Circle; [2] AABB; [3] OBB; [4] Polygon; [5] Attractor",
    "[W][A][S][D] move camera",
    "[UP][DOWN] zoom camera in/out",
    "[R] reset camera to center",
    "[T] toggle text; [H] toggle hitboxes",
    "[C] toggle collision points; [G] toggle grid",
    "[HOVER BODY] when paused, for body information",
    "[U] when paused, to update world manually",
];

pub fn render_info_and_benchmark(
    state: &mut SimulationState,
    nr_of_bodies: usize,
    update_time: f32,
    camera_pos: Vector2,
) {
    // show_fps();

    if state.max_update_duration < update_time {
        state.max_update_duration = update_time;
    }

    let benchmark_info: [&str; 4] = [
        &format!("updates: {}", state.nr_of_updates),
        &format!("max update duration: {:.2}", state.max_update_duration),
        &format!("entity count: {}", nr_of_bodies),
        &format!("camera location: {}", camera_pos),
    ];

    for (idx, info) in benchmark_info.iter().enumerate() {
        draw_text(
            info,
            20.,
            30. + (20. * idx as f32),
            FONT_SIZE,
            UI_TEXT_COLOR,
        );
    }

    let inst_pos = screen_width() - 600.;

    // PAUSE AND USAGE
    draw_text(PAUSE_MENU_INFO, inst_pos, 30.0, FONT_SIZE, UI_TEXT_COLOR);

    if state.simulation == Paused {
        for (idx, instruction) in MANUAL.iter().enumerate() {
            draw_text(
                instruction,
                inst_pos,
                50. + (20. * idx as f32),
                FONT_SIZE,
                UI_TEXT_COLOR_PAUSED,
            );
        }
    }
}

#[allow(dead_code)]
fn show_fps() {
    draw_text(&get_fps().to_string(), 10., 10., 20., BLACK);
}
