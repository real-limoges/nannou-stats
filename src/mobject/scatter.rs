use super::{BoundingRect, Mobject, MobjectId, MobjectStyle, to_screen};
use macroquad::prelude::*;

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
    point_colors: Option<Vec<Color>>,
    scale: f32,
    rotation: f32,
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
                stroke_color: Color::new(0.0, 0.0, 0.0, 0.0),
                fill_color: Color::new(0.3, 0.6, 1.0, 0.8),
                stroke_weight: 1.0,
                opacity: 1.0,
            },
            point_colors: None,
            scale: 1.0,
            rotation: 0.0,
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

    pub fn color(mut self, color: Color) -> Self {
        self.style.fill_color = color;
        self
    }

    pub fn stroke_color(mut self, color: Color) -> Self {
        self.style.stroke_color = color;
        self
    }

    pub fn stroke_weight(mut self, weight: f32) -> Self {
        self.style.stroke_weight = weight;
        self
    }

    pub fn point_colors(mut self, colors: Vec<Color>) -> Self {
        self.point_colors = Some(colors);
        self
    }

    /// Color points using a value-to-color mapping function
    pub fn color_by<F>(mut self, values: &[f32], color_fn: F) -> Self
    where
        F: Fn(f32) -> Color,
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
    fn draw(&self, t: f32, screen_center: Vec2) {
        let style = self.style.with_opacity(self.style.opacity);

        // Calculate how many points to show based on animation progress
        let points_to_draw = ((self.points.len() as f32) * t).ceil() as usize;

        for (i, point) in self.points.iter().take(points_to_draw).enumerate() {
            let screen_pos = to_screen(self.center + *point, screen_center);

            // Get color for this point
            let fill_color = self
                .point_colors
                .as_ref()
                .and_then(|colors| colors.get(i).copied())
                .unwrap_or(style.fill_color);

            // Apply opacity to the fill color
            let fill_with_opacity = Color::new(
                fill_color.r,
                fill_color.g,
                fill_color.b,
                fill_color.a * style.opacity,
            );

            match self.marker {
                MarkerShape::Circle => {
                    if fill_with_opacity.a > 0.0 {
                        draw_circle(
                            screen_pos.x,
                            screen_pos.y,
                            self.point_radius,
                            fill_with_opacity,
                        );
                    }

                    if style.stroke_color.a > 0.0 {
                        draw_circle_lines(
                            screen_pos.x,
                            screen_pos.y,
                            self.point_radius,
                            style.stroke_weight,
                            style.stroke_color,
                        );
                    }
                }
                MarkerShape::Square => {
                    let size = self.point_radius * 2.0;
                    let half = self.point_radius;

                    if fill_with_opacity.a > 0.0 {
                        draw_rectangle(
                            screen_pos.x - half,
                            screen_pos.y - half,
                            size,
                            size,
                            fill_with_opacity,
                        );
                    }

                    if style.stroke_color.a > 0.0 {
                        draw_rectangle_lines(
                            screen_pos.x - half,
                            screen_pos.y - half,
                            size,
                            size,
                            style.stroke_weight,
                            style.stroke_color,
                        );
                    }
                }
                MarkerShape::Diamond => {
                    let r = self.point_radius;
                    let points = [
                        vec2(screen_pos.x, screen_pos.y - r),
                        vec2(screen_pos.x + r, screen_pos.y),
                        vec2(screen_pos.x, screen_pos.y + r),
                        vec2(screen_pos.x - r, screen_pos.y),
                    ];

                    if fill_with_opacity.a > 0.0 {
                        // Draw as two triangles
                        draw_triangle(points[0], points[1], points[2], fill_with_opacity);
                        draw_triangle(points[0], points[2], points[3], fill_with_opacity);
                    }

                    if style.stroke_color.a > 0.0 {
                        for j in 0..4 {
                            let next = (j + 1) % 4;
                            draw_line(
                                points[j].x,
                                points[j].y,
                                points[next].x,
                                points[next].y,
                                style.stroke_weight,
                                style.stroke_color,
                            );
                        }
                    }
                }
                MarkerShape::Cross => {
                    let r = self.point_radius;
                    let weight = style.stroke_weight.max(2.0);
                    // Vertical line
                    draw_line(
                        screen_pos.x,
                        screen_pos.y - r,
                        screen_pos.x,
                        screen_pos.y + r,
                        weight,
                        fill_with_opacity,
                    );
                    // Horizontal line
                    draw_line(
                        screen_pos.x - r,
                        screen_pos.y,
                        screen_pos.x + r,
                        screen_pos.y,
                        weight,
                        fill_with_opacity,
                    );
                }
            }
        }
    }

    fn bounding_box(&self) -> BoundingRect {
        if self.points.is_empty() {
            return BoundingRect::from_xy_wh(self.center, Vec2::ZERO);
        }

        let min = self
            .points
            .iter()
            .fold(Vec2::splat(f32::MAX), |acc, p| acc.min(*p));
        let max = self
            .points
            .iter()
            .fold(Vec2::splat(f32::MIN), |acc, p| acc.max(*p));

        BoundingRect::from_corners(self.center + min, self.center + max)
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

    fn scale(&self) -> f32 {
        self.scale
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    fn rotate(&mut self, angle: f32) {
        self.rotation += angle;
    }

    fn set_rotate(&mut self, angle: f32) {
        self.rotation = angle;
    }

    fn id(&self) -> MobjectId {
        self.id
    }

    fn clone_box(&self) -> Box<dyn Mobject> {
        Box::new(self.clone())
    }
}
