use nannou::prelude::*;

pub mod axes;
pub mod shapes;

pub use axes::{Axes2D, Axes3D};
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

/// Core trait for all drawable mathematical objects
pub trait Mobject: Send + Sync {
    fn draw(&self, draw: &Draw, t: f32);
    fn bounding_box(&self) -> Rect;

    fn center(&self) -> Vec2;
    fn set_center(&mut self, pos: Vec2);

    fn opacity(&self) -> f32;
    fn set_opacity(&mut self, opacity: f32);

    fn id(&self) -> MobjectId;
    fn clone_box(&self) -> Box<dyn Mobject>;
}

/// Common properties shared by all mobjects
#[derive(Debug, Clone)]
pub struct MobjectStyle {
    pub stroke_color: Rgba,
    pub fill_color: Rgba,
    pub stroke_weight: f32,
    pub opacity: f32,
}

impl Default for MobjectStyle {
    fn default() -> Self {
        Self {
            stroke_color: rgba(1.0, 1.0, 1.0, 1.0),
            fill_color: rgba(0.0, 0.0, 0.0, 0.0),
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
        style.stroke_color.alpha = self.stroke_color.alpha * opacity;
        style.fill_color.alpha = self.fill_color.alpha * opacity;
        style
    }
}
