use super::{BoundingRect, Mobject, MobjectId, MobjectStyle, to_screen};
use macroquad::prelude::*;

/// 2D coordinate axes with configurable ranges and styling
#[derive(Debug, Clone)]
pub struct Axes2D {
    id: MobjectId,
    center: Vec2,
    x_range: (f32, f32),
    y_range: (f32, f32),
    scale: f32,
    show_ticks: bool,
    tick_spacing: f32,
    style: MobjectStyle,
    rotation: f32,
}

impl Axes2D {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            x_range: (-5.0, 5.0),
            y_range: (-3.0, 3.0),
            scale: 50.0,
            show_ticks: true,
            tick_spacing: 1.0,
            style: MobjectStyle {
                stroke_color: Color::new(0.7, 0.7, 0.7, 1.0),
                stroke_weight: 2.0,
                ..Default::default()
            },
            rotation: 0f32,
        }
    }

    pub fn x_range(mut self, min: f32, max: f32) -> Self {
        self.x_range = (min, max);
        self
    }

    pub fn y_range(mut self, min: f32, max: f32) -> Self {
        self.y_range = (min, max);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn at(mut self, pos: Vec2) -> Self {
        self.center = pos;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.stroke_color = color;
        self
    }

    pub fn show_ticks(mut self, show: bool) -> Self {
        self.show_ticks = show;
        self
    }

    pub fn tick_spacing(mut self, spacing: f32) -> Self {
        self.tick_spacing = spacing;
        self
    }

    /// Convert a point from coordinate space to screen space
    pub fn to_screen(&self, point: Vec2) -> Vec2 {
        self.center + point * self.scale
    }

    /// Convert a point from screen space to coordinate space
    pub fn from_screen(&self, point: Vec2) -> Vec2 {
        (point - self.center) / self.scale
    }
}

impl Default for Axes2D {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for Axes2D {
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);

        // X-axis (draw from negative to positive)
        let x_start = self.center + vec2(self.x_range.0 * self.scale, 0.0);
        let x_end_full = self.center + vec2(self.x_range.1 * self.scale, 0.0);
        let x_end = x_start.lerp(x_end_full, t);

        let screen_x_start = to_screen(x_start, screen_center);
        let screen_x_end = to_screen(x_end, screen_center);

        draw_line(
            screen_x_start.x,
            screen_x_start.y,
            screen_x_end.x,
            screen_x_end.y,
            style.stroke_weight,
            style.stroke_color,
        );

        // Y-axis
        let y_start = self.center + vec2(0.0, self.y_range.0 * self.scale);
        let y_end_full = self.center + vec2(0.0, self.y_range.1 * self.scale);
        let y_end = y_start.lerp(y_end_full, t);

        let screen_y_start = to_screen(y_start, screen_center);
        let screen_y_end = to_screen(y_end, screen_center);

        draw_line(
            screen_y_start.x,
            screen_y_start.y,
            screen_y_end.x,
            screen_y_end.y,
            style.stroke_weight,
            style.stroke_color,
        );

        // Draw ticks if enabled and t is far enough along
        if self.show_ticks && t > 0.5 {
            let tick_opacity = ((t - 0.5) * 2.0).min(1.0);
            let tick_color = Color::new(
                style.stroke_color.r,
                style.stroke_color.g,
                style.stroke_color.b,
                style.stroke_color.a * tick_opacity,
            );
            let tick_size = 5.0;

            // X-axis ticks
            let mut x = self.x_range.0;
            while x <= self.x_range.1 {
                if x.abs() > 0.001 {
                    let pos = self.center + vec2(x * self.scale, 0.0);
                    let screen_pos = to_screen(pos, screen_center);
                    // Tick is vertical, so we flip the y offset
                    draw_line(
                        screen_pos.x,
                        screen_pos.y - tick_size,
                        screen_pos.x,
                        screen_pos.y + tick_size,
                        1.0,
                        tick_color,
                    );
                }
                x += self.tick_spacing;
            }

            // Y-axis ticks
            let mut y = self.y_range.0;
            while y <= self.y_range.1 {
                if y.abs() > 0.001 {
                    let pos = self.center + vec2(0.0, y * self.scale);
                    let screen_pos = to_screen(pos, screen_center);
                    draw_line(
                        screen_pos.x - tick_size,
                        screen_pos.y,
                        screen_pos.x + tick_size,
                        screen_pos.y,
                        1.0,
                        tick_color,
                    );
                }
                y += self.tick_spacing;
            }
        }
    }

    fn bounding_box(&self) -> BoundingRect {
        let min = self.center + vec2(self.x_range.0 * self.scale, self.y_range.0 * self.scale);
        let max = self.center + vec2(self.x_range.1 * self.scale, self.y_range.1 * self.scale);
        BoundingRect::from_corners(min, max)
    }

    fn center(&self) -> Vec2 {
        self.center
    }

    fn set_center(&mut self, pos: Vec2) {
        self.center = pos;
    }

    fn opacity(&self) -> f32 {
        self.style.opacity
    }

    fn set_opacity(&mut self, opacity: f32) {
        self.style.opacity = opacity;
    }

    fn scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    fn set_rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }

    fn id(&self) -> MobjectId {
        self.id
    }

    fn clone_box(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}

/// 3D coordinate axes with isometric projection
#[derive(Debug, Clone)]
pub struct Axes3D {
    id: MobjectId,
    center: Vec2,
    x_range: (f32, f32),
    y_range: (f32, f32),
    z_range: (f32, f32),
    scale: f32,
    rotation: f32, // rotation angle for 3D effect
    style: MobjectStyle,
}

impl Axes3D {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            x_range: (-3.0, 3.0),
            y_range: (-3.0, 3.0),
            z_range: (-2.0, 2.0),
            scale: 50.0,
            rotation: 0.0,
            style: MobjectStyle {
                stroke_color: Color::new(0.5, 0.5, 0.5, 0.8),
                stroke_weight: 2.0,
                ..Default::default()
            },
        }
    }

    pub fn x_range(mut self, min: f32, max: f32) -> Self {
        self.x_range = (min, max);
        self
    }

    pub fn y_range(mut self, min: f32, max: f32) -> Self {
        self.y_range = (min, max);
        self
    }

    pub fn z_range(mut self, min: f32, max: f32) -> Self {
        self.z_range = (min, max);
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn at(mut self, pos: Vec2) -> Self {
        self.center = pos;
        self
    }

    pub fn rotation(mut self, angle: f32) -> Self {
        self.rotation = angle;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.stroke_color = color;
        self
    }

    /// Project a 3D point to 2D screen coordinates (isometric-style)
    pub fn project(&self, point: Vec3) -> Vec2 {
        // Simple isometric projection with rotation
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

        // Rotate around Y axis
        let x_rot = point.x * cos_r - point.z * sin_r;
        let z_rot = point.x * sin_r + point.z * cos_r;

        // Project to 2D (isometric-ish)
        let x_2d = x_rot * 0.866 - point.y * 0.5; // cos(30) and projection factor
        let y_2d = z_rot * 0.5 + point.y * 0.866; // sin(30) and projection factor

        self.center + vec2(x_2d, y_2d) * self.scale
    }
}

impl Default for Axes3D {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for Axes3D {
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);

        let x_color = Color::new(1.0, 0.3, 0.3, style.stroke_color.a); // Red-ish
        let y_color = Color::new(0.3, 1.0, 0.3, style.stroke_color.a); // Green-ish
        let z_color = Color::new(0.3, 0.3, 1.0, style.stroke_color.a); // Blue-ish

        // X-axis
        let x_start = self.project(vec3(self.x_range.0, 0.0, 0.0));
        let x_end_full = self.project(vec3(self.x_range.1, 0.0, 0.0));
        let x_end = x_start.lerp(x_end_full, t);

        let screen_x_start = to_screen(x_start, screen_center);
        let screen_x_end = to_screen(x_end, screen_center);

        draw_line(
            screen_x_start.x,
            screen_x_start.y,
            screen_x_end.x,
            screen_x_end.y,
            style.stroke_weight,
            x_color,
        );

        // Y-axis
        let y_start = self.project(vec3(0.0, self.y_range.0, 0.0));
        let y_end_full = self.project(vec3(0.0, self.y_range.1, 0.0));
        let y_end = y_start.lerp(y_end_full, t);

        let screen_y_start = to_screen(y_start, screen_center);
        let screen_y_end = to_screen(y_end, screen_center);

        draw_line(
            screen_y_start.x,
            screen_y_start.y,
            screen_y_end.x,
            screen_y_end.y,
            style.stroke_weight,
            y_color,
        );

        // Z-axis
        let z_start = self.project(vec3(0.0, 0.0, self.z_range.0));
        let z_end_full = self.project(vec3(0.0, 0.0, self.z_range.1));
        let z_end = z_start.lerp(z_end_full, t);

        let screen_z_start = to_screen(z_start, screen_center);
        let screen_z_end = to_screen(z_end, screen_center);

        draw_line(
            screen_z_start.x,
            screen_z_start.y,
            screen_z_end.x,
            screen_z_end.y,
            style.stroke_weight,
            z_color,
        );
    }

    fn bounding_box(&self) -> BoundingRect {
        // Approximate bounding box based on all axis endpoints
        let points = [
            self.project(vec3(self.x_range.0, 0.0, 0.0)),
            self.project(vec3(self.x_range.1, 0.0, 0.0)),
            self.project(vec3(0.0, self.y_range.0, 0.0)),
            self.project(vec3(0.0, self.y_range.1, 0.0)),
            self.project(vec3(0.0, 0.0, self.z_range.0)),
            self.project(vec3(0.0, 0.0, self.z_range.1)),
        ];

        let min = points
            .iter()
            .fold(Vec2::splat(f32::MAX), |acc, p| acc.min(*p));
        let max = points
            .iter()
            .fold(Vec2::splat(f32::MIN), |acc, p| acc.max(*p));

        BoundingRect::from_corners(min, max)
    }

    fn center(&self) -> Vec2 {
        self.center
    }

    fn set_center(&mut self, pos: Vec2) {
        self.center = pos;
    }

    fn opacity(&self) -> f32 {
        self.style.opacity
    }

    fn set_opacity(&mut self, opacity: f32) {
        self.style.opacity = opacity;
    }

    fn scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    fn set_rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }

    fn id(&self) -> MobjectId {
        self.id
    }

    fn clone_box(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}
