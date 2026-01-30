use macroquad::prelude::*;
use std::collections::HashMap;

use crate::animation::Animation;
use crate::mobject::{Mobject, MobjectId};
use crate::timeline::Timeline;

/// Camera controls the view transformation
#[derive(Debug, Clone)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

/// A scene contains mobjects and a timeline of animations
pub struct Scene {
    mobjects: HashMap<MobjectId, Box<dyn Mobject>>,
    timeline: Timeline,
    camera: Camera,
    background: Color,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            mobjects: HashMap::new(),
            timeline: Timeline::new(),
            camera: Camera::default(),
            background: Color::new(0.0, 0.0, 0.0, 1.0), // Black background
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn add(&mut self, mobject: impl Mobject + 'static) -> MobjectId {
        let id = mobject.id();
        self.mobjects.insert(id, Box::new(mobject));
        id
    }

    pub fn with(mut self, mobject: impl Mobject + 'static) -> Self {
        self.add(mobject);
        self
    }

    pub fn remove(&mut self, id: MobjectId) -> Option<Box<dyn Mobject>> {
        self.mobjects.remove(&id)
    }

    pub fn get(&self, id: MobjectId) -> Option<&dyn Mobject> {
        match self.mobjects.get(&id) {
            Some(b) => Some(b.as_ref()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: MobjectId) -> Option<&mut dyn Mobject> {
        match self.mobjects.get_mut(&id) {
            Some(b) => Some(b.as_mut()),
            None => None,
        }
    }

    pub fn play(&mut self, animation: impl Animation + 'static) -> &mut Self {
        self.timeline.play(animation);
        self
    }

    pub fn play_together(&mut self, animations: Vec<Box<dyn Animation>>) -> &mut Self {
        self.timeline.play_together(animations);
        self
    }

    pub fn wait(&mut self, duration: f32) -> &mut Self {
        self.timeline.wait(duration);
        self
    }

    pub fn timeline(&self) -> &Timeline {
        &self.timeline
    }

    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.timeline
    }

    pub fn duration(&self) -> f32 {
        self.timeline.total_duration()
    }

    pub fn background_color(&self) -> Color {
        self.background
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_position(mut self, pos: Vec2) -> Self {
        self.camera.position = pos;
        self
    }

    pub fn camera_zoom(mut self, zoom: f32) -> Self {
        self.camera.zoom = zoom;
        self
    }

    /// Draw all mobjects at a specific time
    /// screen_center is the center of the screen in screen coordinates (for coordinate transform)
    pub fn draw_at(&mut self, time: f32, screen_center: Vec2) {
        // Apply all active animations
        let active = self.timeline.active_at(time);

        for (entry, t) in &active {
            let target_id = entry.animation.target_id();
            if let Some(mobject) = self.mobjects.get_mut(&target_id) {
                entry.animation.apply(mobject.as_mut(), *t);
            }
        }

        // Draw all mobjects
        for (id, mobject) in &self.mobjects {
            // Get draw progress for Create-type animations
            let draw_progress = self.timeline.draw_progress_for(*id, time);
            mobject.draw(draw_progress, screen_center);
        }
    }

    pub fn mobject_ids(&self) -> Vec<MobjectId> {
        self.mobjects.keys().copied().collect()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
