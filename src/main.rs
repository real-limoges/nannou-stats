use macroquad::prelude::*;
use maquette::prelude::{
    Axes2D, Circle as MobjectCircle, Easing, FadeIn, MoveTo, Scene, to_screen,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Maquette Demo".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = Scene::new().background(Color::new(0.0, 0.0, 0.0, 1.0));

    let axes = Axes2D::new()
        .x_range(-4.0, 4.0)
        .y_range(-3.0, 3.0)
        .scale(80.0)
        .color(Color::new(0.5, 0.5, 0.5, 1.0));

    let circle = MobjectCircle::new()
        .radius(30.0)
        .at(vec2(-200.0, 0.0))
        .color(Color::new(0.3, 0.6, 1.0, 1.0))
        .fill(Color::new(0.3, 0.6, 1.0, 0.3));

    let axes_id = scene.add(axes);
    let circle_id = scene.add(circle);

    // Build the animation timeline
    scene
        .play(FadeIn::new(axes_id).duration(1.0))
        .wait(0.2)
        .play(FadeIn::new(circle_id).duration(0.5))
        .wait(0.3)
        .play(
            MoveTo::new(circle_id, vec2(200.0, 100.0))
                .duration(1.5)
                .easing(Easing::EaseInOutCubic),
        )
        .wait(0.2)
        .play(
            MoveTo::new(circle_id, vec2(-100.0, -80.0))
                .duration(1.0)
                .easing(Easing::EaseOutBack),
        )
        .wait(0.2)
        .play(
            MoveTo::new(circle_id, vec2(0.0, 0.0))
                .duration(0.8)
                .easing(Easing::Smooth),
        );

    println!("Animation duration: {:.2}s", scene.duration());
    println!("Press SPACE to play/pause, R to restart");

    let mut time = 0.0f32;
    let mut playing = true;

    loop {
        // Input handling
        if is_key_pressed(KeyCode::Space) {
            playing = !playing;
            println!("Playing: {}", playing);
        }

        if is_key_pressed(KeyCode::R) {
            time = 0.0;
            // Reset mobject states
            if let Some(m) = scene.get_mut(circle_id) {
                m.set_opacity(0.0);
                m.set_center(vec2(-200.0, 0.0));
            }
            if let Some(m) = scene.get_mut(axes_id) {
                m.set_opacity(0.0);
            }
            println!("Restarted");
        }

        // Update time
        if playing {
            time += get_frame_time();

            if time > scene.duration() + 1.0 {
                time = 0.0;

                if let Some(m) = scene.get_mut(circle_id) {
                    m.set_opacity(0.0);
                    m.set_center(vec2(-200.0, 0.0));
                }
                if let Some(m) = scene.get_mut(axes_id) {
                    m.set_opacity(0.0);
                }
            }
        }

        // Calculate screen center for coordinate transform
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        // Clear background
        clear_background(scene.background_color());

        let total_time = time;

        // Calculate axes opacity
        let axes_opacity = if total_time < 1.0 {
            Easing::Smooth.apply(total_time / 1.0)
        } else {
            1.0
        };

        // Calculate circle opacity
        let circle_fade_start = 1.2;
        let circle_fade_end = 1.7;
        let circle_opacity = if total_time < circle_fade_start {
            0.0
        } else if total_time < circle_fade_end {
            Easing::Smooth.apply((total_time - circle_fade_start) / 0.5)
        } else {
            1.0
        };

        // Calculate circle position
        let move1_start = 2.0;
        let move1_end = 3.5;
        let move2_start = 3.7;
        let move2_end = 4.7;
        let move3_start = 4.9;
        let move3_end = 5.7;

        let start_pos = vec2(-200.0, 0.0);
        let pos1 = vec2(200.0, 100.0);
        let pos2 = vec2(-100.0, -80.0);
        let pos3 = vec2(0.0, 0.0);

        let circle_pos = if total_time < move1_start {
            start_pos
        } else if total_time < move1_end {
            let t = (total_time - move1_start) / 1.5;
            start_pos.lerp(pos1, Easing::EaseInOutCubic.apply(t))
        } else if total_time < move2_start {
            pos1
        } else if total_time < move2_end {
            let t = (total_time - move2_start) / 1.0;
            pos1.lerp(pos2, Easing::EaseOutBack.apply(t))
        } else if total_time < move3_start {
            pos2
        } else if total_time < move3_end {
            let t = (total_time - move3_start) / 0.8;
            pos2.lerp(pos3, Easing::Smooth.apply(t))
        } else {
            pos3
        };

        // Draw axes
        let scale = 80.0;
        let axes_color = Color::new(0.5, 0.5, 0.5, axes_opacity);

        // X-axis
        let x_start = to_screen(vec2(-4.0 * scale, 0.0), screen_center);
        let x_end = to_screen(vec2(4.0 * scale, 0.0), screen_center);
        draw_line(x_start.x, x_start.y, x_end.x, x_end.y, 2.0, axes_color);

        // Y-axis
        let y_start = to_screen(vec2(0.0, -3.0 * scale), screen_center);
        let y_end = to_screen(vec2(0.0, 3.0 * scale), screen_center);
        draw_line(y_start.x, y_start.y, y_end.x, y_end.y, 2.0, axes_color);

        // Draw ticks (fade in after axes)
        if axes_opacity > 0.5 {
            let tick_alpha = ((axes_opacity - 0.5) * 2.0).min(1.0);
            let tick_color = Color::new(0.5, 0.5, 0.5, tick_alpha);

            for i in -4..=4 {
                if i != 0 {
                    let x = i as f32 * scale;
                    let tick_pos = to_screen(vec2(x, 0.0), screen_center);
                    draw_line(
                        tick_pos.x,
                        tick_pos.y - 5.0,
                        tick_pos.x,
                        tick_pos.y + 5.0,
                        1.0,
                        tick_color,
                    );
                }
            }
            for i in -3..=3 {
                if i != 0 {
                    let y = i as f32 * scale;
                    let tick_pos = to_screen(vec2(0.0, y), screen_center);
                    draw_line(
                        tick_pos.x - 5.0,
                        tick_pos.y,
                        tick_pos.x + 5.0,
                        tick_pos.y,
                        1.0,
                        tick_color,
                    );
                }
            }
        }

        // Draw circle
        if circle_opacity > 0.0 {
            let circle_stroke = Color::new(0.3, 0.6, 1.0, circle_opacity);
            let circle_fill = Color::new(0.3, 0.6, 1.0, circle_opacity * 0.3);

            let screen_pos = to_screen(circle_pos, screen_center);

            draw_circle(screen_pos.x, screen_pos.y, 30.0, circle_fill);
            draw_circle_lines(screen_pos.x, screen_pos.y, 30.0, 2.0, circle_stroke);
        }

        // Draw progress bar
        let progress = (time / scene.duration()).min(1.0);
        let bar_width = 200.0;
        let bar_y = screen_height() - 40.0;
        let bar_x = screen_center.x - bar_width / 2.0;

        draw_rectangle(bar_x, bar_y, bar_width, 4.0, Color::new(0.3, 0.3, 0.3, 1.0));
        draw_rectangle(bar_x, bar_y, bar_width * progress, 4.0, Color::new(0.3, 0.6, 1.0, 1.0));

        // Draw time text
        let time_text = format!(
            "{:.1}s / {:.1}s",
            time.min(scene.duration()),
            scene.duration()
        );
        let text_dimensions = measure_text(&time_text, None, 14, 1.0);
        draw_text(
            &time_text,
            screen_center.x - text_dimensions.width / 2.0,
            bar_y + 20.0,
            14.0,
            WHITE,
        );

        next_frame().await
    }
}
