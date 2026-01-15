//! # Nannou-Stats
//!
//! A Manim-inspired visualization framework built on nannou.
//!
//! ## Quick Start
/*
use nannou_stats::prelude::*;

let mut scene = Scene::new();

let circle_id = scene.add(Circle::new().radius(50.0).color(BLUE));
let axes_id = scene.add(Axes2D::default());

scene.play(FadeIn::new(axes_id).duration(1.0));
scene.play(FadeIn::new(circle_id).duration(0.5));
scene.play(MoveTo::new(circle_id, vec2(100.0, 50.0)).duration(1.0));
*/

pub mod animation;
pub mod mobject;
pub mod render;
pub mod scene;
pub mod state;
pub mod timeline;

pub mod prelude {
    pub use nannou::prelude::*;

    // Mobjects
    pub use crate::mobject::{Arrow, Circle, Line, Rectangle};
    pub use crate::mobject::{Axes2D, Axes3D};
    pub use crate::mobject::{Mobject, MobjectId, MobjectStyle};

    // Animations
    pub use crate::animation::{Animation, Easing};
    pub use crate::animation::{Create, FadeIn, FadeOut};
    pub use crate::animation::{MoveTo, Rotate, Scale, Shift};

    // Scene and Timeline
    pub use crate::scene::{Camera, Scene};
    pub use crate::timeline::Timeline;

    // Rendering
    pub use crate::render::{FrameInfo, OutputFormat, RenderConfig, Renderable, Renderer};

    // Data
    pub use crate::state::Dataset;
}

// Re-export key types at crate root
pub use animation::Animation;
pub use mobject::{Mobject, MobjectId};
pub use render::{RenderConfig, Renderer};
pub use scene::Scene;
pub use state::Dataset;
pub use timeline::Timeline;
