use super::{Mobject, MobjectId, MobjectStyle};
use nannou::prelude::*;

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
                stroke_color: rgba(0.7, 0.7, 0.7, 1.0),
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

    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    pub fn at(mut self, pos: Vec2) -> Self {
        self.center = pos;
        self
    }

    pub fn color(mut self, color: impl Into<Rgba>) -> Self {
        self.style.stroke_color = color.into();
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
    fn draw(&self, draw: &Draw, t: f32) {
        let style = self.style.with_opacity(self.style.opacity);

        // Calculate how much of each axis to draw based on t
        let _x_len = self.x_range.1 - self.x_range.0;
        let _y_len = self.y_range.1 - self.y_range.0;

        // X-axis (draw from negative to positive)
        let x_start = self.center + vec2(self.x_range.0 * self.scale, 0.0);
        let x_end_full = self.center + vec2(self.x_range.1 * self.scale, 0.0);
        let x_end = x_start.lerp(x_end_full, t);

        draw.line()
            .start(x_start)
            .end(x_end)
            .color(style.stroke_color)
            .stroke_weight(style.stroke_weight);

        // Y-axis
        let y_start = self.center + vec2(0.0, self.y_range.0 * self.scale);
        let y_end_full = self.center + vec2(0.0, self.y_range.1 * self.scale);
        let y_end = y_start.lerp(y_end_full, t);

        draw.line()
            .start(y_start)
            .end(y_end)
            .color(style.stroke_color)
            .stroke_weight(style.stroke_weight);

        // Draw ticks if enabled and t is far enough along
        if self.show_ticks && t > 0.5 {
            let tick_opacity = ((t - 0.5) * 2.0).min(1.0);
            let tick_color = rgba(
                style.stroke_color.red,
                style.stroke_color.green,
                style.stroke_color.blue,
                style.stroke_color.alpha * tick_opacity,
            );
            let tick_size = 5.0;

            // X-axis ticks
            let mut x = self.x_range.0;
            while x <= self.x_range.1 {
                if x.abs() > 0.001 {
                    let pos = self.center + vec2(x * self.scale, 0.0);
                    draw.line()
                        .start(pos + vec2(0.0, -tick_size))
                        .end(pos + vec2(0.0, tick_size))
                        .color(tick_color)
                        .stroke_weight(1.0);
                }
                x += self.tick_spacing;
            }

            // Y-axis ticks
            let mut y = self.y_range.0;
            while y <= self.y_range.1 {
                if y.abs() > 0.001 {
                    let pos = self.center + vec2(0.0, y * self.scale);
                    draw.line()
                        .start(pos + vec2(-tick_size, 0.0))
                        .end(pos + vec2(tick_size, 0.0))
                        .color(tick_color)
                        .stroke_weight(1.0);
                }
                y += self.tick_spacing;
            }
        }
    }

    fn bounding_box(&self) -> Rect {
        let min = self.center + vec2(self.x_range.0 * self.scale, self.y_range.0 * self.scale);
        let max = self.center + vec2(self.x_range.1 * self.scale, self.y_range.1 * self.scale);
        Rect::from_corners(min, max)
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
                stroke_color: rgba(0.5, 0.5, 0.5, 0.8),
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

    pub fn color(mut self, color: impl Into<Rgba>) -> Self {
        self.style.stroke_color = color.into();
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
    fn draw(&self, draw: &Draw, t: f32) {
        let style = self.style.with_opacity(self.style.opacity);

        // Colors for each axis
        let x_color = rgba(1.0, 0.3, 0.3, style.stroke_color.alpha); // Red-ish
        let y_color = rgba(0.3, 1.0, 0.3, style.stroke_color.alpha); // Green-ish
        let z_color = rgba(0.3, 0.3, 1.0, style.stroke_color.alpha); // Blue-ish

        // X-axis
        let x_start = self.project(vec3(self.x_range.0, 0.0, 0.0));
        let x_end_full = self.project(vec3(self.x_range.1, 0.0, 0.0));
        let x_end = x_start.lerp(x_end_full, t);

        draw.line()
            .start(x_start)
            .end(x_end)
            .color(x_color)
            .stroke_weight(style.stroke_weight);

        // Y-axis
        let y_start = self.project(vec3(0.0, self.y_range.0, 0.0));
        let y_end_full = self.project(vec3(0.0, self.y_range.1, 0.0));
        let y_end = y_start.lerp(y_end_full, t);

        draw.line()
            .start(y_start)
            .end(y_end)
            .color(y_color)
            .stroke_weight(style.stroke_weight);

        // Z-axis
        let z_start = self.project(vec3(0.0, 0.0, self.z_range.0));
        let z_end_full = self.project(vec3(0.0, 0.0, self.z_range.1));
        let z_end = z_start.lerp(z_end_full, t);

        draw.line()
            .start(z_start)
            .end(z_end)
            .color(z_color)
            .stroke_weight(style.stroke_weight);
    }

    fn bounding_box(&self) -> Rect {
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

        Rect::from_corners(min, max)
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

    fn id(&self) -> MobjectId {
        self.id
    }

    fn clone_box(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}
