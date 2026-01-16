use nannou::prelude::*;
use std::path::PathBuf;
use std::process::Command;

use crate::scene::Scene;

#[derive(Debug, Clone, Copy, Default)]
pub enum OutputFormat {
    #[default]
    Frames,
    Mp4,
}

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub output_dir: PathBuf,
    pub format: OutputFormat,
    pub video_filename: String,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 30,
            output_dir: PathBuf::from("output"),
            format: OutputFormat::Frames,
            video_filename: "output.mp4".to_string(),
        }
    }
}

impl RenderConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    pub fn output_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.output_dir = path.into();
        self
    }

    pub fn format(mut self, format: OutputFormat) -> Self {
        self.format = format;
        self
    }

    pub fn video_filename(mut self, name: impl Into<String>) -> Self {
        self.video_filename = name.into();
        self
    }

    pub fn total_frames(&self, duration: f32) -> u32 {
        (duration * self.fps as f32).ceil() as u32
    }
}

pub struct Renderer;

impl Renderer {
    /// Render a scene to frames (Simplified)
    /// I have a lot of xtra stuff to do  here
    /// integrates into nannou's App/Model pattern
    pub fn render_frames(
        scene: &mut Scene,
        config: &RenderConfig,
        _draw_fn: impl Fn(&Draw, &mut Scene, f32),
    ) -> std::io::Result<Vec<PathBuf>> {
        std::fs::create_dir_all(&config.output_dir)?;

        let duration = scene.duration();
        let total_frames = config.total_frames(duration);
        let frame_duration = 1.0 / config.fps as f32;

        let mut frame_paths = Vec::new();

        for frame in 0..total_frames {
            let _time = frame as f32 * frame_duration;
            let frame_path = config.output_dir.join(format!("frame_{:05}.png", frame));

            // The actual drawing would be done by the draw function (which accesses nannou)
            // draw_fn(&draw, scene, time);

            frame_paths.push(frame_path);
        }

        Ok(frame_paths)
    }

    /// MP4 using ffmpeg
    pub fn frames_to_video(config: &RenderConfig) -> std::io::Result<PathBuf> {
        let input_pattern = config.output_dir.join("frame_%05d.png");
        let output_path = config.output_dir.join(&config.video_filename);

        let status = Command::new("ffmpeg")
            .args([
                "-y", // Overwrite output
                "-framerate",
                &config.fps.to_string(),
                "-i",
                input_pattern.to_str().unwrap(),
                "-c:v",
                "libx264",
                "-pix_fmt",
                "yuv420p",
                "-crf",
                "18", // Quality (lower = better, 18 is visually lossless)
                output_path.to_str().unwrap(),
            ])
            .status()?;

        if status.success() {
            Ok(output_path)
        } else {
            Err(std::io::Error::other("ffmpeg failed to create video"))
        }
    }

    pub fn ffmpeg_available() -> bool {
        Command::new("ffmpeg")
            .arg("-version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

pub trait Renderable {
    fn render_frame(&mut self, draw: &Draw, time: f32);

    fn duration(&self) -> f32;
}

impl Renderable for Scene {
    fn render_frame(&mut self, draw: &Draw, time: f32) {
        draw.background().color(self.background_color());

        self.draw_at(draw, time);
    }

    fn duration(&self) -> f32 {
        self.timeline().total_duration()
    }
}

/// Frame info for rendering callbacks
#[derive(Debug, Clone, Copy)]
pub struct FrameInfo {
    pub frame_number: u32,
    pub total_frames: u32,
    pub time: f32,
    pub duration: f32,
}

impl FrameInfo {
    pub fn progress(&self) -> f32 {
        self.frame_number as f32 / self.total_frames as f32
    }
}
