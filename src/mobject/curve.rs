use super::{Mobject, MobjectId, MobjectStyle};
use nannou::prelude::*;

/// A curve mobject for drawing smooth lines through points.
/// Useful for plotting GAM fits, regression lines, and function graphs.
#[derive(Debug, Clone)]
pub struct Curve {
    id: MobjectId,
    center: Vec2,
    points: Vec<Vec2>,
    style: MobjectStyle,
}

impl Curve {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            points: Vec::new(),
            style: MobjectStyle {
                stroke_color: rgba(1.0, 0.4, 0.4, 1.0),
                fill_color: rgba(0.0, 0.0, 0.0, 0.0),
                stroke_weight: 2.5,
                opacity: 1.0,
            },
        }
    }

    pub fn from_points(points: Vec<Vec2>) -> Self {
        Self::new().points(points)
    }

    pub fn from_function<F>(f: F, x_min: f32, x_max: f32, samples: usize) -> Self
    where
        F: Fn(f32) -> f32,
    {
        let points = (0..samples)
            .map(|i| {
                let t = i as f32 / (samples - 1) as f32;
                let x = x_min + t * (x_max - x_min);
                vec2(x, f(x))
            })
            .collect();
        Self::new().points(points)
    }

    pub fn points(mut self, points: Vec<Vec2>) -> Self {
        self.points = points;
        self
    }

    /// Set the center offset (useful with Axes2D)
    pub fn at(mut self, pos: Vec2) -> Self {
        self.center = pos;
        self
    }

    pub fn color(mut self, color: impl Into<Rgba>) -> Self {
        self.style.stroke_color = color.into();
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn get_points(&self) -> &[Vec2] {
        &self.points
    }
}

impl Default for Curve {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for Curve {
    fn draw(&self, draw: &Draw, t: f32) {
        if self.points.len() < 2 {
            return;
        }

        let style = self.style.with_opacity(self.style.opacity);

        // Calculate how many points to draw based on animation progress
        let points_to_draw = ((self.points.len() as f32) * t).ceil() as usize;
        let points_to_draw = points_to_draw.max(2).min(self.points.len());

        // Offset points by center
        let screen_points: Vec<Vec2> = self.points[..points_to_draw]
            .iter()
            .map(|p| self.center + *p)
            .collect();

        draw.polyline()
            .weight(style.stroke_weight)
            .points(screen_points)
            .color(style.stroke_color);
    }

    fn bounding_box(&self) -> Rect {
        if self.points.is_empty() {
            return Rect::from_xy_wh(self.center, Vec2::ZERO);
        }

        let min = self
            .points
            .iter()
            .fold(Vec2::splat(f32::MAX), |acc, p| acc.min(*p));
        let max = self
            .points
            .iter()
            .fold(Vec2::splat(f32::MIN), |acc, p| acc.max(*p));

        Rect::from_corners(self.center + min, self.center + max)
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

/// A confidence band mobject for showing uncertainty around a curve.
/// Commonly used to display GAM confidence intervals.
#[derive(Debug, Clone)]
pub struct ConfidenceBand {
    id: MobjectId,
    center: Vec2,
    upper: Vec<Vec2>,
    lower: Vec<Vec2>,
    style: MobjectStyle,
}

impl ConfidenceBand {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            upper: Vec::new(),
            lower: Vec::new(),
            style: MobjectStyle {
                stroke_color: rgba(0.0, 0.0, 0.0, 0.0),
                fill_color: rgba(1.0, 0.4, 0.4, 0.2),
                stroke_weight: 0.0,
                opacity: 1.0,
            },
        }
    }

    pub fn from_bounds(lower: Vec<Vec2>, upper: Vec<Vec2>) -> Self {
        Self::new().bounds(lower, upper)
    }

    /// Create a confidence band from a center curve and symmetric error
    pub fn from_curve_with_error(center_points: &[Vec2], error: &[f32]) -> Self {
        let lower: Vec<Vec2> = center_points
            .iter()
            .zip(error.iter())
            .map(|(p, &e)| vec2(p.x, p.y - e))
            .collect();

        let upper: Vec<Vec2> = center_points
            .iter()
            .zip(error.iter())
            .map(|(p, &e)| vec2(p.x, p.y + e))
            .collect();

        Self::new().bounds(lower, upper)
    }

    /// Set the upper and lower bounds
    pub fn bounds(mut self, lower: Vec<Vec2>, upper: Vec<Vec2>) -> Self {
        self.lower = lower;
        self.upper = upper;
        self
    }

    /// Set the center offset (useful with Axes2D)
    pub fn at(mut self, pos: Vec2) -> Self {
        self.center = pos;
        self
    }

    pub fn fill(mut self, color: impl Into<Rgba>) -> Self {
        self.style.fill_color = color.into();
        self
    }

    pub fn stroke_color(mut self, color: impl Into<Rgba>) -> Self {
        self.style.stroke_color = color.into();
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }
}

impl Default for ConfidenceBand {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for ConfidenceBand {
    fn draw(&self, draw: &Draw, t: f32) {
        if self.lower.len() < 2 || self.upper.len() < 2 {
            return;
        }

        let style = self.style.with_opacity(self.style.opacity);

        // Calculate how many points to draw based on animation progress
        let n = self.lower.len().min(self.upper.len());
        let points_to_draw = ((n as f32) * t).ceil() as usize;
        let points_to_draw = points_to_draw.max(2).min(n);

        // Create a polygon: lower points forward, then upper points backward
        let mut polygon_points: Vec<Vec2> = Vec::with_capacity(points_to_draw * 2);

        // Add lower points (left to right)
        for p in self.lower[..points_to_draw].iter() {
            polygon_points.push(self.center + *p);
        }

        // Add upper points (right to left)
        for p in self.upper[..points_to_draw].iter().rev() {
            polygon_points.push(self.center + *p);
        }

        if style.fill_color.alpha > 0.0 && polygon_points.len() >= 3 {
            draw.polygon()
                .points(polygon_points.clone())
                .color(style.fill_color);
        }

        if style.stroke_color.alpha > 0.0 {
            let lower_screen: Vec<Vec2> = self.lower[..points_to_draw]
                .iter()
                .map(|p| self.center + *p)
                .collect();
            let upper_screen: Vec<Vec2> = self.upper[..points_to_draw]
                .iter()
                .map(|p| self.center + *p)
                .collect();

            draw.polyline()
                .weight(style.stroke_weight)
                .points(lower_screen)
                .color(style.stroke_color);

            draw.polyline()
                .weight(style.stroke_weight)
                .points(upper_screen)
                .color(style.stroke_color);
        }
    }

    fn bounding_box(&self) -> Rect {
        if self.lower.is_empty() && self.upper.is_empty() {
            return Rect::from_xy_wh(self.center, Vec2::ZERO);
        }

        let all_points = self.lower.iter().chain(self.upper.iter());
        let min = all_points
            .clone()
            .fold(Vec2::splat(f32::MAX), |acc, p| acc.min(*p));
        let max = all_points.fold(Vec2::splat(f32::MIN), |acc, p| acc.max(*p));

        Rect::from_corners(self.center + min, self.center + max)
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
