use nannou::prelude::*;
use nannou_stats::prelude::*;

/// Application model
struct Model {
    scene: Scene,
    time: f32,
    playing: bool,
    circle_id: MobjectId,
    axes_id: MobjectId,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1280, 720)
        .title("Nannou-Stats Demo")
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let mut scene = Scene::new().background(rgba(0.0, 0.0, 0.0, 1.0));

    let axes = Axes2D::new()
        .x_range(-4.0, 4.0)
        .y_range(-3.0, 3.0)
        .scale(80.0)
        .color(rgba(0.5, 0.5, 0.5, 1.0));

    let circle = Circle::new()
        .radius(30.0)
        .at(vec2(-200.0, 0.0))
        .color(rgba(0.3, 0.6, 1.0, 1.0))
        .fill(rgba(0.3, 0.6, 1.0, 0.3));

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

    Model {
        scene,
        time: 0.0,
        playing: true,
        circle_id,
        axes_id,
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.playing {
        model.time += update.since_last.as_secs_f32();

        if model.time > model.scene.duration() + 1.0 {
            model.time = 0.0;

            if let Some(m) = model.scene.get_mut(model.circle_id) {
                m.set_opacity(0.0);
                m.set_center(vec2(-200.0, 0.0));
            }
            if let Some(m) = model.scene.get_mut(model.axes_id) {
                m.set_opacity(0.0);
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Clear background
    draw.background().color(model.scene.background_color());

    let total_time = model.time;

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
    let axes_color = rgba(0.5, 0.5, 0.5, axes_opacity);

    // X-axis
    draw.line()
        .start(vec2(-4.0 * scale, 0.0))
        .end(vec2(4.0 * scale, 0.0))
        .color(axes_color)
        .stroke_weight(2.0);

    // Y-axis
    draw.line()
        .start(vec2(0.0, -3.0 * scale))
        .end(vec2(0.0, 3.0 * scale))
        .color(axes_color)
        .stroke_weight(2.0);

    // Draw ticks (fade in after axes)
    if axes_opacity > 0.5 {
        let tick_alpha = ((axes_opacity - 0.5) * 2.0).min(1.0);
        let tick_color = rgba(0.5, 0.5, 0.5, tick_alpha);

        for i in -4..=4 {
            if i != 0 {
                let x = i as f32 * scale;
                draw.line()
                    .start(vec2(x, -5.0))
                    .end(vec2(x, 5.0))
                    .color(tick_color)
                    .stroke_weight(1.0);
            }
        }
        for i in -3..=3 {
            if i != 0 {
                let y = i as f32 * scale;
                draw.line()
                    .start(vec2(-5.0, y))
                    .end(vec2(5.0, y))
                    .color(tick_color)
                    .stroke_weight(1.0);
            }
        }
    }

    // Draw circle
    if circle_opacity > 0.0 {
        let circle_stroke = rgba(0.3, 0.6, 1.0, circle_opacity);
        let circle_fill = rgba(0.3, 0.6, 1.0, circle_opacity * 0.3);

        draw.ellipse()
            .xy(circle_pos)
            .radius(30.0)
            .color(circle_fill);

        draw.ellipse()
            .xy(circle_pos)
            .radius(30.0)
            .no_fill()
            .stroke(circle_stroke)
            .stroke_weight(2.0);
    }

    // Draw progress bar
    let progress = (model.time / model.scene.duration()).min(1.0);
    let bar_width = 200.0;
    let bar_y = -320.0;

    draw.rect()
        .xy(vec2(0.0, bar_y))
        .w_h(bar_width, 4.0)
        .color(rgba(0.3, 0.3, 0.3, 1.0));

    draw.rect()
        .xy(vec2(-bar_width / 2.0 + (bar_width * progress) / 2.0, bar_y))
        .w_h(bar_width * progress, 4.0)
        .color(rgba(0.3, 0.6, 1.0, 1.0));

    // Draw time text
    draw.text(&format!(
        "{:.1}s / {:.1}s",
        model.time.min(model.scene.duration()),
        model.scene.duration()
    ))
    .xy(vec2(0.0, bar_y - 20.0))
    .color(WHITE)
    .font_size(14);

    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.playing = !model.playing;
            println!("Playing: {}", model.playing);
        }
        Key::R => {
            model.time = 0.0;
            // Reset mobject states
            if let Some(m) = model.scene.get_mut(model.circle_id) {
                m.set_opacity(0.0);
                m.set_center(vec2(-200.0, 0.0));
            }
            if let Some(m) = model.scene.get_mut(model.axes_id) {
                m.set_opacity(0.0);
            }
            println!("Restarted");
        }
        _ => {}
    }
}
