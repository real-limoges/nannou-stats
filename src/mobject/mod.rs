use macroquad::prelude::*;

pub mod axes;
pub mod curve;
pub mod scatter;
pub mod shapes;

pub use axes::{Axes2D, Axes3D};
pub use curve::{ConfidenceBand, Curve};
pub use scatter::{MarkerShape, ScatterPlot};
pub use shapes::{Arrow, Circle, Line, Rectangle};

/// Unique identifier for mobjects in a scene
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MobjectId(pub u64);

impl MobjectId {
    pub fn new() -> Self {
        use std::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for MobjectId {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple rectangle type for bounding boxes (replaces nannou's Rect)
#[derive(Debug, Clone, Copy)]
pub struct BoundingRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl BoundingRect {
    pub fn from_xy_wh(center: Vec2, size: Vec2) -> Self {
        Self {
            x: center.x - size.x / 2.0,
            y: center.y - size.y / 2.0,
            w: size.x,
            h: size.y,
        }
    }

    pub fn from_corners(min: Vec2, max: Vec2) -> Self {
        Self {
            x: min.x,
            y: min.y,
            w: max.x - min.x,
            h: max.y - min.y,
        }
    }
}

/// Core trait for all drawable mathematical objects
pub trait Mobject: Send + Sync {
    /// Draw the mobject. Uses macroquad's global immediate-mode draw functions.
    /// The `t` parameter is for partial drawing (0.0-1.0) in Create animations.
    /// The `screen_center` is used to offset coordinates (center-origin to top-left origin).
    fn draw(&self, t: f32, screen_center: Vec2);
    fn bounding_box(&self) -> BoundingRect;

    fn center(&self) -> Vec2;
    fn set_center(&mut self, pos: Vec2);

    fn opacity(&self) -> f32;
    fn set_opacity(&mut self, opacity: f32);

    fn scale(&self) -> f32;
    fn set_scale(&mut self, scale: f32);

    fn rotate(&mut self, angle: f32);
    fn set_rotate(&mut self, angle: f32);

    fn id(&self) -> MobjectId;
    fn clone_box(&self) -> Box<dyn Mobject>;
}

/// Common properties shared by all mobjects
#[derive(Debug, Clone)]
pub struct MobjectStyle {
    pub stroke_color: Color,
    pub fill_color: Color,
    pub stroke_weight: f32,
    pub opacity: f32,
}

impl Default for MobjectStyle {
    fn default() -> Self {
        Self {
            stroke_color: Color::new(1.0, 1.0, 1.0, 1.0),
            fill_color: Color::new(0.0, 0.0, 0.0, 0.0),
            stroke_weight: 2.0,
            opacity: 1.0,
        }
    }
}

impl MobjectStyle {
    /// Apply opacity to the style colors
    pub fn with_opacity(&self, opacity: f32) -> Self {
        let mut style = self.clone();
        style.opacity = opacity;
        style.stroke_color.a = self.stroke_color.a * opacity;
        style.fill_color.a = self.fill_color.a * opacity;
        style
    }
}

/// Helper to convert from center-origin coordinates to screen coordinates
/// In macroquad, (0,0) is top-left and Y increases downward
pub fn to_screen(pos: Vec2, screen_center: Vec2) -> Vec2 {
    vec2(screen_center.x + pos.x, screen_center.y - pos.y)
}
