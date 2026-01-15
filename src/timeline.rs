use crate::animation::{Animation, AnimationEntry};
use crate::mobject::MobjectId;

/// Timeline manages the sequencing of animations
#[derive(Clone, Default)]
pub struct Timeline {
    entries: Vec<AnimationEntry>,
    current_time: f32,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            current_time: 0.0,
        }
    }

    /// Add an animation to play sequentially (after all previous animations)
    pub fn play(&mut self, animation: impl Animation + 'static) {
        let entry = AnimationEntry::new(Box::new(animation), self.current_time);
        self.current_time = entry.end_time();
        self.entries.push(entry);
    }

    /// Add multiple animations to play in parallel (all start at current time)
    pub fn play_together(&mut self, animations: Vec<Box<dyn Animation>>) {
        let start_time = self.current_time;
        let mut max_end_time = self.current_time;

        for animation in animations {
            let entry = AnimationEntry::new(animation, start_time);
            if entry.end_time() > max_end_time {
                max_end_time = entry.end_time();
            }
            self.entries.push(entry);
        }

        self.current_time = max_end_time;
    }

    pub fn wait(&mut self, duration: f32) {
        self.current_time += duration;
    }

    pub fn total_duration(&self) -> f32 {
        self.entries
            .iter()
            .map(|e| e.end_time())
            .fold(0.0_f32, |a, b| a.max(b))
    }

    pub fn entries(&self) -> &[AnimationEntry] {
        &self.entries
    }

    pub fn active_at(&self, time: f32) -> Vec<(&AnimationEntry, f32)> {
        self.entries
            .iter()
            .filter_map(|entry| {
                if time >= entry.start_time && time < entry.end_time() {
                    let local_t = (time - entry.start_time) / entry.animation.duration();
                    Some((entry, local_t))
                } else if time >= entry.end_time() {
                    // Animation completed - return with t=1.0
                    Some((entry, 1.0))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn animation_progress(&self, entry: &AnimationEntry, time: f32) -> f32 {
        if time < entry.start_time {
            0.0
        } else if time >= entry.end_time() {
            1.0
        } else {
            (time - entry.start_time) / entry.animation.duration()
        }
    }

    /// Get the draw progress for a mobject at a given time
    /// This is used by Create/Uncreate animations
    pub fn draw_progress_for(&self, mobject_id: MobjectId, time: f32) -> f32 {
        // Find the most recent Create animation for this mobject
        for entry in self.entries.iter().rev() {
            if entry.animation.target_id() == mobject_id {
                let progress = self.animation_progress(entry, time);
                return progress;
            }
        }
        // Default to fully drawn if no animation found
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::FadeIn;
    use crate::mobject::MobjectId;

    #[test]
    fn test_sequential_animations() {
        let mut timeline = Timeline::new();
        let id = MobjectId::new();

        timeline.play(FadeIn::new(id).duration(1.0));
        timeline.play(FadeIn::new(id).duration(0.5));

        assert!((timeline.total_duration() - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_wait() {
        let mut timeline = Timeline::new();
        let id = MobjectId::new();

        timeline.play(FadeIn::new(id).duration(1.0));
        timeline.wait(0.5);
        timeline.play(FadeIn::new(id).duration(1.0));

        assert!((timeline.total_duration() - 2.5).abs() < 0.001);
    }
}
