use super::{BoundingRect, Mobject, MobjectId, MobjectStyle, to_screen};
use macroquad::prelude::*;

/// A circle mobject
#[derive(Debug, Clone)]
pub struct Circle {
    id: MobjectId,
    center: Vec2,
    radius: f32,
    style: MobjectStyle,
    scale: f32,
    rotation: f32,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            radius: 1.0,
            style: MobjectStyle::default(),
            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.radius = r;
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

    pub fn fill(mut self, color: Color) -> Self {
        self.style.fill_color = color;
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for Circle {
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);
        let screen_pos = to_screen(self.center, screen_center);

        // For partial drawing (Create animation), only draw arc up to t
        let resolution = 64;
        let points_to_draw = ((resolution as f32) * t).ceil() as usize;

        if points_to_draw == 0 {
            return;
        }

        // Draw fill if present
        if style.fill_color.a > 0.0 {
            draw_circle(screen_pos.x, screen_pos.y, self.radius, style.fill_color);
        }

        // Draw stroke
        if t >= 1.0 {
            draw_circle_lines(
                screen_pos.x,
                screen_pos.y,
                self.radius,
                style.stroke_weight,
                style.stroke_color,
            );
        } else {
            // Partial stroke for animation - draw as connected line segments
            let points: Vec<Vec2> = (0..=points_to_draw)
                .map(|i| {
                    let angle = (i as f32 / resolution as f32) * std::f32::consts::TAU;
                    let local_pos = self.center + Vec2::new(angle.cos(), angle.sin()) * self.radius;
                    to_screen(local_pos, screen_center)
                })
                .collect();

            for i in 0..points.len().saturating_sub(1) {
                draw_line(
                    points[i].x,
                    points[i].y,
                    points[i + 1].x,
                    points[i + 1].y,
                    style.stroke_weight,
                    style.stroke_color,
                );
            }
        }
    }

    fn bounding_box(&self) -> BoundingRect {
        BoundingRect::from_xy_wh(self.center, Vec2::splat(self.radius * 2.0))
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

/// A line segment mobject
#[derive(Debug, Clone)]
pub struct Line {
    id: MobjectId,
    start: Vec2,
    end: Vec2,
    style: MobjectStyle,
    scale: f32,
    rotation: f32,
}

impl Line {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            id: MobjectId::new(),
            start,
            end,
            style: MobjectStyle::default(),
            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn from_to(mut self, start: Vec2, end: Vec2) -> Self {
        self.start = start;
        self.end = end;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.stroke_color = color;
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }
}

impl Mobject for Line {
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);
        let current_end = self.start.lerp(self.end, t);

        let screen_start = to_screen(self.start, screen_center);
        let screen_end = to_screen(current_end, screen_center);

        draw_line(
            screen_start.x,
            screen_start.y,
            screen_end.x,
            screen_end.y,
            style.stroke_weight,
            style.stroke_color,
        );
    }

    fn bounding_box(&self) -> BoundingRect {
        let min = self.start.min(self.end);
        let max = self.start.max(self.end);
        BoundingRect::from_corners(min, max)
    }

    fn center(&self) -> Vec2 {
        (self.start + self.end) / 2.0
    }

    fn set_center(&mut self, pos: Vec2) {
        let offset = pos - self.center();
        self.start += offset;
        self.end += offset;
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

/// A rectangle mobject
#[derive(Debug, Clone)]
pub struct Rectangle {
    id: MobjectId,
    center: Vec2,
    width: f32,
    height: f32,
    style: MobjectStyle,
    scale: f32,
    rotation: f32,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            width: 2.0,
            height: 1.0,
            style: MobjectStyle::default(),
            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
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

    pub fn fill(mut self, color: Color) -> Self {
        self.style.fill_color = color;
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for Rectangle {
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);
        let hw = self.width / 2.0;
        let hh = self.height / 2.0;

        // Corner points in center-origin coordinates
        let corners = [
            self.center + vec2(-hw, -hh),
            self.center + vec2(hw, -hh),
            self.center + vec2(hw, hh),
            self.center + vec2(-hw, hh),
        ];

        // Convert to screen coordinates
        let screen_corners: Vec<Vec2> = corners
            .iter()
            .map(|c| to_screen(*c, screen_center))
            .collect();

        // Draw fill
        if style.fill_color.a > 0.0 {
            let screen_pos = to_screen(self.center, screen_center);
            draw_rectangle(
                screen_pos.x - hw,
                screen_pos.y - hh,
                self.width,
                self.height,
                style.fill_color,
            );
        }

        // Draw stroke with animation progress
        let perimeter = 2.0 * (self.width + self.height);
        let draw_length = perimeter * t;

        if t >= 1.0 {
            // Draw complete rectangle outline
            for i in 0..4 {
                let next = (i + 1) % 4;
                draw_line(
                    screen_corners[i].x,
                    screen_corners[i].y,
                    screen_corners[next].x,
                    screen_corners[next].y,
                    style.stroke_weight,
                    style.stroke_color,
                );
            }
        } else {
            // Partial stroke
            let mut remaining = draw_length;
            let edges = [
                (corners[0], corners[1]),
                (corners[1], corners[2]),
                (corners[2], corners[3]),
                (corners[3], corners[0]),
            ];

            for (start, end) in edges {
                let edge_len = (end - start).length();
                if remaining <= 0.0 {
                    break;
                }
                let t_edge = (remaining / edge_len).min(1.0);
                let current_end = start.lerp(end, t_edge);

                let screen_start = to_screen(start, screen_center);
                let screen_end = to_screen(current_end, screen_center);

                draw_line(
                    screen_start.x,
                    screen_start.y,
                    screen_end.x,
                    screen_end.y,
                    style.stroke_weight,
                    style.stroke_color,
                );

                remaining -= edge_len;
            }
        }
    }

    fn bounding_box(&self) -> BoundingRect {
        BoundingRect::from_xy_wh(self.center, vec2(self.width, self.height))
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

/// An arrow mobject (line with arrowhead)
#[derive(Debug, Clone)]
pub struct Arrow {
    id: MobjectId,
    start: Vec2,
    end: Vec2,
    tip_size: f32,
    style: MobjectStyle,
    scale: f32,
    rotation: f32,
}

impl Arrow {
    pub fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            id: MobjectId::new(),
            start,
            end,
            tip_size: 10.0,
            style: MobjectStyle::default(),
            scale: 1.0,
            rotation: 0.0,
        }
    }

    pub fn tip_size(mut self, size: f32) -> Self {
        self.tip_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.style.stroke_color = color;
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }
}

impl Mobject for Arrow {
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);
        let current_end = self.start.lerp(self.end, t);

        let screen_start = to_screen(self.start, screen_center);
        let screen_end = to_screen(current_end, screen_center);

        // Draw line
        draw_line(
            screen_start.x,
            screen_start.y,
            screen_end.x,
            screen_end.y,
            style.stroke_weight,
            style.stroke_color,
        );

        // Draw arrowhead only when line reaches near end
        if t > 0.9 {
            let dir = (self.end - self.start).normalize();
            let perp = vec2(-dir.y, dir.x);

            let tip = current_end;
            let left = tip - dir * self.tip_size + perp * self.tip_size * 0.5;
            let right = tip - dir * self.tip_size - perp * self.tip_size * 0.5;

            let screen_tip = to_screen(tip, screen_center);
            let screen_left = to_screen(left, screen_center);
            let screen_right = to_screen(right, screen_center);

            draw_triangle(screen_tip, screen_left, screen_right, style.stroke_color);
        }
    }

    fn bounding_box(&self) -> BoundingRect {
        let min = self.start.min(self.end);
        let max = self.start.max(self.end);
        BoundingRect::from_corners(min, max)
    }

    fn center(&self) -> Vec2 {
        (self.start + self.end) / 2.0
    }

    fn set_center(&mut self, pos: Vec2) {
        let offset = pos - self.center();
        self.start += offset;
        self.end += offset;
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
