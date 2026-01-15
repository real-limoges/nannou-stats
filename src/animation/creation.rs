use super::{Animation, Easing};
use crate::mobject::{Mobject, MobjectId};

/// Fade in animation - opacity from 0 to 1
#[derive(Debug, Clone)]
pub struct FadeIn {
    target: MobjectId,
    duration: f32,
    easing: Easing,
}

impl FadeIn {
    pub fn new(target: MobjectId) -> Self {
        Self {
            target,
            duration: 1.0,
            easing: Easing::Smooth,
        }
    }

    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs;
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

impl Animation for FadeIn {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, mobject: &mut dyn Mobject, t: f32) {
        let eased_t = self.easing.apply(t);
        mobject.set_opacity(eased_t);
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

/// Fade out animation - opacity from 1 to 0
#[derive(Debug, Clone)]
pub struct FadeOut {
    target: MobjectId,
    duration: f32,
    easing: Easing,
}

impl FadeOut {
    pub fn new(target: MobjectId) -> Self {
        Self {
            target,
            duration: 1.0,
            easing: Easing::Smooth,
        }
    }

    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs;
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

impl Animation for FadeOut {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, mobject: &mut dyn Mobject, t: f32) {
        let eased_t = self.easing.apply(t);
        mobject.set_opacity(1.0 - eased_t);
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

/// Create animation - progressively draws the mobject's stroke
/// This animation doesn't modify the mobject directly; instead,
/// the mobject's draw() method receives the t parameter
#[derive(Debug, Clone)]
pub struct Create {
    target: MobjectId,
    duration: f32,
    easing: Easing,
    /// Current progress stored for the mobject to query during draw
    progress: f32,
}

impl Create {
    pub fn new(target: MobjectId) -> Self {
        Self {
            target,
            duration: 1.0,
            easing: Easing::Smooth,
            progress: 0.0,
        }
    }

    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs;
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Get the current drawing progress (used by renderer)
    pub fn progress(&self) -> f32 {
        self.progress
    }
}

impl Animation for Create {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, _mobject: &mut dyn Mobject, t: f32) {
        // Note: Create animation works differently - the progress is used
        // during drawing rather than modifying mobject properties.
        // The scene/renderer will pass t to the mobject's draw() method.
        let _ = self.easing.apply(t);
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

/// Uncreate animation - reverse of Create (stroke erases progressively)
#[derive(Debug, Clone)]
pub struct Uncreate {
    target: MobjectId,
    duration: f32,
    easing: Easing,
}

impl Uncreate {
    pub fn new(target: MobjectId) -> Self {
        Self {
            target,
            duration: 1.0,
            easing: Easing::Smooth,
        }
    }

    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs;
        self
    }

    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

impl Animation for Uncreate {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, _mobject: &mut dyn Mobject, t: f32) {
        // Similar to Create but in reverse - handled during draw
        let _ = self.easing.apply(t);
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}
