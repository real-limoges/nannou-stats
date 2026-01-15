use std::f32::consts::PI;

/// Easing functions for smooth animations
#[derive(Debug, Clone, Copy, Default)]
pub enum Easing {
    /// Constant speed
    Linear,
    /// Smooth start and end (default, like Manim)
    #[default]
    Smooth,
    /// Accelerate from zero
    EaseInQuad,
    /// Decelerate to zero
    EaseOutQuad,
    /// Accelerate then decelerate
    EaseInOutQuad,
    /// Stronger acceleration
    EaseInCubic,
    /// Stronger deceleration
    EaseOutCubic,
    /// Stronger accel/decel
    EaseInOutCubic,
    /// Sinusoidal ease in/out
    EaseInOutSine,
    /// Exponential ease in
    EaseInExpo,
    /// Exponential ease out
    EaseOutExpo,
    /// Overshoot then settle (elastic feel)
    EaseOutBack,
    /// Bounce at the end
    EaseOutBounce,
}

impl Easing {
    /// Apply the easing function to a linear time value t (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Easing::Linear => t,

            // Smooth (Manim's default) - similar to ease in/out sine
            Easing::Smooth => smooth(t),

            // Quadratic
            Easing::EaseInQuad => t * t,
            Easing::EaseOutQuad => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOutQuad => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }

            // Cubic
            Easing::EaseInCubic => t * t * t,
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }

            // Sinusoidal
            Easing::EaseInOutSine => -(((t * PI).cos() - 1.0) / 2.0),

            // Exponential
            Easing::EaseInExpo => {
                if t == 0.0 {
                    0.0
                } else {
                    (2.0_f32).powf(10.0 * t - 10.0)
                }
            }
            Easing::EaseOutExpo => {
                if t == 1.0 {
                    1.0
                } else {
                    1.0 - (2.0_f32).powf(-10.0 * t)
                }
            }

            // Back (overshoot)
            Easing::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }

            // Bounce
            Easing::EaseOutBounce => ease_out_bounce(t),
        }
    }
}

/// Manim-style smooth function (S-curve)
fn smooth(t: f32) -> f32 {
    // Using smoothstep (Hermite interpolation)
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Bounce easing helper
fn ease_out_bounce(t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;

    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        let t = t - 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        let t = t - 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_bounds() {
        let easings = [
            Easing::Linear,
            Easing::Smooth,
            Easing::EaseInQuad,
            Easing::EaseOutQuad,
            Easing::EaseInOutQuad,
        ];

        for easing in easings {
            assert!((easing.apply(0.0) - 0.0).abs() < 0.001, "{:?} at 0", easing);
            assert!((easing.apply(1.0) - 1.0).abs() < 0.001, "{:?} at 1", easing);
        }
    }

    #[test]
    fn test_smooth_midpoint() {
        // Smooth should be 0.5 at t=0.5
        let mid = Easing::Smooth.apply(0.5);
        assert!((mid - 0.5).abs() < 0.001);
    }
}
