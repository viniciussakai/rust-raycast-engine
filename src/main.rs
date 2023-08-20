use std::f32::consts::PI;

use grid::grid;
use macroquad::prelude::*;

fn rotate_vector(vec: &mut Vec2, angle: f32) -> Vec2 {
    Vec2 {
        x: vec.x * angle.cos() - vec.y * angle.sin(),
        y: vec.x * angle.sin() + vec.y * angle.cos(),
    }
}

fn map(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
#[macroquad::main("BasicShapes")]
async fn main() {
    const LIGHTBLUE: Color = color_u8!(117, 163, 201, 1);
    const DARKRED: Color = color_u8!(143, 13, 48, 1);

    let game_map = grid![
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        [1, 1, 0, 0, 1, 0, 0, 0, 0, 1]
        [1, 0, 0, 0, 0, 0, 0, 1, 0, 1]
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        [1, 0, 0, 0, 0, 0, 0, 0, 0, 1]
        [1, 1, 1, 0, 0, 0, 0, 0, 1, 1]
        [1, 0, 0, 1, 0, 0, 0, 0, 0, 1]
        [1, 0, 0, 0, 1, 0, 0, 0, 0, 1]
        [1, 1, 0, 0, 1, 0, 0, 0, 1, 1]
        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    ];

    let mut pos = vec2(5.0, 5.0);
    let mut dir = vec2(0.0, -1.0);
    let mut camera_plane = vec2(0.66, 0.0);
    let movement_speed = 5.0;
    let mut rotation_speed = 0.0;
    let mut strafe_velocity = vec2(0.0, 0.0);
    let mut rotation_intensity = 1.0;
    let mut velocity = vec2(0.0, 0.0);

    set_cursor_grab(true);

    loop {
        clear_background(LIGHTBLUE);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., WHITE);

        draw_rectangle(
            0.0,
            screen_height() / 2.0,
            screen_width(),
            screen_height(),
            LIGHTGRAY,
        );

        for pixel in 0..screen_width() as i32 {
            let multipleir = 2.0 * (pixel as f32 / screen_width()) - 1.0;
            let camera_pixel = camera_plane * multipleir;
            let ray_dir = dir + camera_pixel;

            let delta_dist_x: f32;
            let delta_dist_y: f32;

            match ray_dir {
                Vec2 { x, .. } if x == 0.0 => {
                    delta_dist_x = 1.0;
                    delta_dist_y = 0.0
                }

                Vec2 { y, .. } if y == 0.0 => {
                    delta_dist_y = 1.0;
                    delta_dist_x = 0.0;
                }

                _ => {
                    delta_dist_x = (1.0 / ray_dir.x).abs();
                    delta_dist_y = (1.0 / ray_dir.y).abs();
                }
            }

            let map_pos = pos.floor();

            let mut step_x: f32 = 0.0;
            let mut step_y: f32 = 0.0;

            let mut perpendicular_dist: f32 = 0.0;

            let dist_side_x = match ray_dir {
                Vec2 { x, .. } if x < 0.0 => {
                    step_x = -1.0;
                    (pos.x - map_pos.x) * delta_dist_x
                }
                Vec2 { x, .. } if x > 0.0 => {
                    step_x = 1.0;
                    (map_pos.x + 1.0 - pos.x) * delta_dist_x
                }
                _ => 0.0,
            };

            let dist_side_y = match ray_dir {
                Vec2 { y, .. } if y < 0.0 => {
                    step_y = -1.0;
                    (pos.y - map_pos.y) * delta_dist_y
                }
                Vec2 { y, .. } if y > 0.0 => {
                    step_y = 1.0;
                    (map_pos.y + 1.0 - pos.y) * delta_dist_y
                }
                _ => 0.0,
            };

            let mut hit = false;
            let mut dda_line_x = dist_side_x;
            let mut dda_line_y = dist_side_y;

            let mut wall_pos = map_pos.clone();
            let mut hit_side: i32 = 0;

            while !hit {
                if dda_line_x < dda_line_y {
                    wall_pos.x += step_x;
                    dda_line_x += delta_dist_x;
                    hit_side = 0;
                } else {
                    wall_pos.y += step_y;
                    dda_line_y += delta_dist_y;
                    hit_side = 1;
                }

                match game_map.get(wall_pos.x as usize, wall_pos.y as usize) {
                    Some(value) if *value > 0 => {
                        hit = true;
                    }
                    _ => {}
                }
            }

            let mut line_color: Color = color_u8!(0, 0, 0, 0);

            match hit_side {
                0 => {
                    perpendicular_dist =
                        (wall_pos.x - pos.x + ((1.0 - step_x) / 2.0)).abs() / ray_dir.x;
                    line_color = RED;
                }
                1 => {
                    perpendicular_dist =
                        (wall_pos.y - pos.y + ((1.0 - step_y) / 2.0)).abs() / ray_dir.y;
                    line_color = DARKBROWN;
                }
                _ => {}
            }

            let wall_line_height = screen_height() / perpendicular_dist;
            let line_start_y = screen_height() / 2.0 - wall_line_height / 2.0;
            let line_end_y = screen_height() / 2.0 + wall_line_height / 2.0;

            draw_line(
                pixel as f32,
                line_start_y,
                pixel as f32,
                line_end_y,
                1.0,
                line_color,
            )
        }

        if is_key_down(KeyCode::W) {
            velocity = dir.clone();
            velocity = velocity * movement_speed;
        } else if is_key_down(KeyCode::S) {
            velocity = dir.clone();
            velocity = velocity * -movement_speed;
        } else {
            velocity = velocity * 0.0;
        }

        if is_key_down(KeyCode::D) {
            strafe_velocity = dir.clone();
            strafe_velocity = rotate_vector(&mut strafe_velocity, PI / 2.0);
            strafe_velocity = strafe_velocity * movement_speed;
        } else if is_key_down(KeyCode::A) {
            strafe_velocity = dir.clone();
            strafe_velocity = rotate_vector(&mut strafe_velocity, -PI / 2.0);
            strafe_velocity = strafe_velocity * movement_speed;
        } else {
            strafe_velocity = strafe_velocity * 0.0;
        }

        let moused = mouse_delta_position();
        let mouse_intensity = map(moused.x, 0.0, 1.0, 0.0, 5.0);

        if moused.x > 0.0 {
            rotation_intensity = mouse_intensity.abs();
            rotation_speed = -3.0;
        } else if moused.x < 0.0 {
            rotation_speed = 3.0;
            rotation_intensity = mouse_intensity.abs();
        } else {
            rotation_intensity = 0.0;
            rotation_speed = 0.0;
        }

        strafe_velocity = strafe_velocity * (1.0 / 75.0);
        velocity = velocity * (1.0 / 75.0);
        pos = pos + velocity + strafe_velocity;

        dir = rotate_vector(&mut dir, (rotation_speed / 75.0) * rotation_intensity);
        camera_plane = rotate_vector(
            &mut camera_plane,
            (rotation_speed / 75.0) * rotation_intensity,
        );

        draw_line(
            screen_width() / 2.0 - 5.0,
            screen_height() / 2.0,
            screen_width() / 2.0 + 5.0,
            screen_height() / 2.0,
            3.0,
            BLACK,
        );

        draw_line(
            screen_width() / 2.0,
            screen_height() / 2.0 - 5.0,
            screen_width() / 2.0,
            screen_height() / 2.0 + 5.0,
            3.0,
            BLACK,
        );

        show_mouse(false);

        if is_key_down(KeyCode::Escape) {
            set_cursor_grab(false);
        }

        next_frame().await
    }
}
