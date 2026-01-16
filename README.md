 # nannou-stats

A Manim-inspired visualization framework built on [nannou](https://nannou.cc/) for creating animated statistical and mathematical visualizations in Rust.

## Project Status

**Version:** 0.1.0 (Early Development)

### Implemented Features

#### Mobjects (Mathematical Objects)
- **Shapes**: `Circle`, `Line`, `Rectangle`, `Arrow`
  - Builder pattern for configuration
  - Customizable stroke color, fill color, and stroke weight
  - Support for partial drawing (animation-aware)
- **Coordinate Systems**: `Axes2D`, `Axes3D`
  - Configurable axis ranges and scale
  - Tick marks with customizable spacing
  - 3D axes with isometric projection and rotation
  - Coordinate conversion utilities (`to_screen`, `from_screen`)

#### Animation System
- **Creation Animations**: `FadeIn`, `FadeOut`, `Create`, `Uncreate`
- **Transform Animations**: `MoveTo`, `Shift`
- **Placeholder Animations**: `Scale`, `Rotate` (structure in place, need trait extensions)
- **13 Easing Functions**:
  - `Linear`, `Smooth` (default)
  - Quadratic: `EaseInQuad`, `EaseOutQuad`, `EaseInOutQuad`
  - Cubic: `EaseInCubic`, `EaseOutCubic`, `EaseInOutCubic`
  - Other: `EaseInOutSine`, `EaseInExpo`, `EaseOutExpo`, `EaseOutBack`, `EaseOutBounce`

#### Scene & Timeline
- Scene management with camera support (position, zoom)
- Background color configuration
- Mobject add/remove/get operations
- Timeline with sequential and parallel animation support
- Wait times between animations
- Fluent API: `scene.play(...).wait(...).play(...)`

#### Rendering
- Configurable render settings (resolution, FPS)
- Frame-by-frame rendering support
- FFmpeg integration for MP4 video output
- `Renderable` trait for custom rendering

#### Data Utilities
- `Dataset` struct for statistical data
- Sample data generators:
  - GAM-style wavy surface data
  - Linear regression data with noise
  - Clustered data points

### Architecture

```
src/
├── lib.rs          # Prelude and module exports
├── main.rs         # Demo application
├── scene.rs        # Scene and Camera
├── timeline.rs     # Animation sequencing
├── state.rs        # Dataset utilities
├── render.rs       # Rendering configuration
├── mobject/
│   ├── mod.rs      # Mobject trait and MobjectStyle
│   ├── shapes.rs   # Circle, Line, Rectangle, Arrow
│   └── axes.rs     # Axes2D, Axes3D
└── animation/
    ├── mod.rs      # Animation trait and AnimationEntry
    ├── creation.rs # FadeIn, FadeOut, Create, Uncreate
    ├── transform.rs# MoveTo, Shift, Scale, Rotate
    └── easing.rs   # Easing functions
```

## Quick Start

```rust
use nannou_stats::prelude::*;

let mut scene = Scene::new().background(rgba(0.0, 0.0, 0.0, 1.0));

// Add mobjects
let axes_id = scene.add(Axes2D::new().x_range(-4.0, 4.0).y_range(-3.0, 3.0));
let circle_id = scene.add(Circle::new().radius(30.0).color(rgba(0.3, 0.6, 1.0, 1.0)));

// Build animation timeline
scene
    .play(FadeIn::new(axes_id).duration(1.0))
    .wait(0.2)
    .play(FadeIn::new(circle_id).duration(0.5))
    .play(MoveTo::new(circle_id, vec2(100.0, 50.0)).duration(1.0).easing(Easing::EaseInOutCubic));
```

## Running the Demo

```bash
cargo run
```

**Controls:**
- `SPACE` - Play/Pause
- `R` - Restart animation

## TODO / Roadmap

- [ ] Complete `Scale` and `Rotate` animations (add trait methods)
- [ ] Text/LaTeX rendering support
- [ ] Graphs and function plotting
- [ ] Statistical chart mobjects (bar charts, scatter plots, histograms)
- [ ] Morphing between mobjects
- [ ] Camera animations (pan, zoom)
- [ ] Improved frame rendering pipeline
- [ ] Documentation and examples

## Dependencies

- `nannou` 0.19.0 - Creative coding framework
- `ffmpeg` (optional) - For video export

## License

MIT
