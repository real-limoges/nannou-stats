use crate::mobject::{Mobject, MobjectId};

pub mod creation;
pub mod easing;
pub mod transform;

pub use creation::{Create, FadeIn, FadeOut};
pub use easing::Easing;
pub use transform::{MoveTo, Rotate, Scale, Shift};

/// Core trait for all animations
pub trait Animation: Send + Sync {
    /// Duration of this animation in seconds
    fn duration(&self) -> f32;

    /// The ID of the mobject this animation targets
    fn target_id(&self) -> MobjectId;

    /// Apply this animation to a mobject at time t (0.0 to 1.0)
    /// Returns the interpolated progress after applying easing
    fn apply(&self, mobject: &mut dyn Mobject, t: f32);

    /// Clone this animation into a boxed trait object
    fn clone_box(&self) -> Box<dyn Animation>;
}

/// Wrapper to store animation with its starting time in a timeline
#[derive(Clone)]
pub struct AnimationEntry {
    pub animation: Box<dyn Animation>,
    pub start_time: f32,
}

impl AnimationEntry {
    pub fn new(animation: Box<dyn Animation>, start_time: f32) -> Self {
        Self {
            animation,
            start_time,
        }
    }

    pub fn end_time(&self) -> f32 {
        self.start_time + self.animation.duration()
    }
}

// Allow cloning boxed animations
impl Clone for Box<dyn Animation> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
