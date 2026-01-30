use super::{Animation, Easing};
use crate::mobject::{Mobject, MobjectId};
use macroquad::prelude::*;

/// Move to an absolute position
#[derive(Debug, Clone)]
pub struct MoveTo {
    target: MobjectId,
    destination: Vec2,
    start_pos: Option<Vec2>,
    duration: f32,
    easing: Easing,
}

impl MoveTo {
    pub fn new(target: MobjectId, destination: Vec2) -> Self {
        Self {
            target,
            destination,
            start_pos: None,
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

impl Animation for MoveTo {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, mobject: &mut dyn Mobject, t: f32) {
        let eased_t = self.easing.apply(t);

        // Capture start position on first application
        let start = self.start_pos.unwrap_or_else(|| mobject.center());

        let new_pos = start.lerp(self.destination, eased_t);
        mobject.set_center(new_pos);
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

/// Shift by a relative offset
#[derive(Debug, Clone)]
pub struct Shift {
    target: MobjectId,
    delta: Vec2,
    start_pos: Option<Vec2>,
    duration: f32,
    easing: Easing,
}

impl Shift {
    pub fn new(target: MobjectId, delta: Vec2) -> Self {
        Self {
            target,
            delta,
            start_pos: None,
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

impl Animation for Shift {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, mobject: &mut dyn Mobject, t: f32) {
        let eased_t = self.easing.apply(t);

        let start = self.start_pos.unwrap_or_else(|| mobject.center());
        let destination = start + self.delta;

        let new_pos = start.lerp(destination, eased_t);
        mobject.set_center(new_pos);
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

/// Scale animation - requires mobjects to implement scaling
#[derive(Debug, Clone)]
pub struct Scale {
    target: MobjectId,
    #[allow(dead_code)]
    factor: f32,
    duration: f32,
    easing: Easing,
}

impl Scale {
    pub fn new(target: MobjectId, factor: f32) -> Self {
        Self {
            target,
            factor,
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

impl Animation for Scale {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, _mobject: &mut dyn Mobject, t: f32) {
        let _eased_t = self.easing.apply(t);
        // 2. Interpolate from 1.0 to self.factor based on eased_t
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}

/// Rotate animation - requires mobjects to implement rotation
#[derive(Debug, Clone)]
pub struct Rotate {
    target: MobjectId,
    #[allow(dead_code)]
    angle: f32, // radians
    duration: f32,
    easing: Easing,
}

impl Rotate {
    pub fn new(target: MobjectId, angle: f32) -> Self {
        Self {
            target,
            angle,
            duration: 1.0,
            easing: Easing::Smooth,
        }
    }

    /// Rotate by degrees instead of radians
    pub fn degrees(target: MobjectId, degrees: f32) -> Self {
        Self::new(target, degrees.to_radians())
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

impl Animation for Rotate {
    fn duration(&self) -> f32 {
        self.duration
    }

    fn target_id(&self) -> MobjectId {
        self.target
    }

    fn apply(&self, _mobject: &mut dyn Mobject, t: f32) {
        let _eased_t = self.easing.apply(t);
        // 2. Interpolate from 0.0 to self.angle based on eased_t
    }

    fn clone_box(&self) -> Box<dyn Animation> {
        Box::new(self.clone())
    }
}
