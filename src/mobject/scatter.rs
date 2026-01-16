use super::{Mobject, MobjectId, MobjectStyle};
use nannou::prelude::*;

/// Point marker shapes for scatter plots
#[derive(Debug, Clone, Copy, Default)]
pub enum MarkerShape {
    #[default]
    Circle,
    Square,
    Diamond,
    Cross,
}

/// A scatter plot mobject for visualizing 2D point data
#[derive(Debug, Clone)]
pub struct ScatterPlot {
    id: MobjectId,
    center: Vec2,
    points: Vec<Vec2>,
    point_radius: f32,
    marker: MarkerShape,
    style: MobjectStyle,
    /// Optional per-point colors (for coloring by value/residual/cluster)
    point_colors: Option<Vec<Rgba>>,
}

impl ScatterPlot {
    pub fn new() -> Self {
        Self {
            id: MobjectId::new(),
            center: Vec2::ZERO,
            points: Vec::new(),
            point_radius: 4.0,
            marker: MarkerShape::Circle,
            style: MobjectStyle {
                stroke_color: rgba(0.0, 0.0, 0.0, 0.0),
                fill_color: rgba(0.3, 0.6, 1.0, 0.8),
                stroke_weight: 1.0,
                opacity: 1.0,
            },
            point_colors: None,
        }
    }

    pub fn from_points(points: Vec<Vec2>) -> Self {
        Self::new().points(points)
    }

    pub fn points(mut self, points: Vec<Vec2>) -> Self {
        self.points = points;
        self
    }

    /// Set the center offset for all points (useful with Axes2D)
    pub fn at(mut self, pos: Vec2) -> Self {
        self.center = pos;
        self
    }

    pub fn point_radius(mut self, radius: f32) -> Self {
        self.point_radius = radius;
        self
    }

    pub fn marker(mut self, shape: MarkerShape) -> Self {
        self.marker = shape;
        self
    }

    pub fn color(mut self, color: impl Into<Rgba>) -> Self {
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

    pub fn point_colors(mut self, colors: Vec<Rgba>) -> Self {
        self.point_colors = Some(colors);
        self
    }

    /// Color points using a value-to-color mapping function
    pub fn color_by<F>(mut self, values: &[f32], color_fn: F) -> Self
    where
        F: Fn(f32) -> Rgba,
    {
        self.point_colors = Some(values.iter().map(|&v| color_fn(v)).collect());
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

impl Default for ScatterPlot {
    fn default() -> Self {
        Self::new()
    }
}

impl Mobject for ScatterPlot {
    fn draw(&self, draw: &Draw, t: f32) {
        let style = self.style.with_opacity(self.style.opacity);

        // Calculate how many points to show based on animation progress
        let points_to_draw = ((self.points.len() as f32) * t).ceil() as usize;

        for (i, point) in self.points.iter().take(points_to_draw).enumerate() {
            let screen_pos = self.center + *point;

            // Get color for this point
            let fill_color = self
                .point_colors
                .as_ref()
                .and_then(|colors| colors.get(i).copied())
                .unwrap_or(style.fill_color);

            // Apply opacity to the fill color
            let fill_with_opacity = rgba(
                fill_color.red,
                fill_color.green,
                fill_color.blue,
                fill_color.alpha * style.opacity,
            );

            match self.marker {
                MarkerShape::Circle => {
                    let mut ellipse = draw.ellipse().xy(screen_pos).radius(self.point_radius);

                    if fill_with_opacity.alpha > 0.0 {
                        ellipse = ellipse.color(fill_with_opacity);
                    } else {
                        ellipse = ellipse.no_fill();
                    }

                    if style.stroke_color.alpha > 0.0 {
                        ellipse
                            .stroke(style.stroke_color)
                            .stroke_weight(style.stroke_weight);
                    }
                }
                MarkerShape::Square => {
                    let size = self.point_radius * 2.0;
                    let mut rect = draw.rect().xy(screen_pos).w_h(size, size);

                    if fill_with_opacity.alpha > 0.0 {
                        rect = rect.color(fill_with_opacity);
                    } else {
                        rect = rect.no_fill();
                    }

                    if style.stroke_color.alpha > 0.0 {
                        rect.stroke(style.stroke_color)
                            .stroke_weight(style.stroke_weight);
                    }
                }
                MarkerShape::Diamond => {
                    let r = self.point_radius;
                    let points = [
                        screen_pos + vec2(0.0, r),
                        screen_pos + vec2(r, 0.0),
                        screen_pos + vec2(0.0, -r),
                        screen_pos + vec2(-r, 0.0),
                    ];

                    if fill_with_opacity.alpha > 0.0 {
                        draw.quad()
                            .points(points[0], points[1], points[2], points[3])
                            .color(fill_with_opacity);
                    }

                    if style.stroke_color.alpha > 0.0 {
                        draw.polyline()
                            .weight(style.stroke_weight)
                            .points([points[0], points[1], points[2], points[3], points[0]])
                            .color(style.stroke_color);
                    }
                }
                MarkerShape::Cross => {
                    let r = self.point_radius;
                    // Vertical line
                    draw.line()
                        .start(screen_pos + vec2(0.0, -r))
                        .end(screen_pos + vec2(0.0, r))
                        .color(fill_with_opacity)
                        .stroke_weight(style.stroke_weight.max(2.0));
                    // Horizontal line
                    draw.line()
                        .start(screen_pos + vec2(-r, 0.0))
                        .end(screen_pos + vec2(r, 0.0))
                        .color(fill_with_opacity)
                        .stroke_weight(style.stroke_weight.max(2.0));
                }
            }
        }
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
