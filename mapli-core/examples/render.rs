//! Render a map using the mapli-core library.
//!
//! This example demonstrates how to render a map using the Mapli Core library. It creates a
//! render pool, registers a style from a URL, sets the camera position, and renders the map to
//! an image. Finally, it saves the rendered image as "map.png".
//!
//! Notably, there is no trace of the wrapped MapLibre Native, as mapli-core abstracts away the
//! underlying implementation details.

use mapli_core::{Camera, ImageSpec, PoolConfig, RenderPool, StaticRequest, Style};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = RenderPool::new(PoolConfig::default())?;
    pool.register_style("demo", Style::Url("https://demotiles.maplibre.org/style.json".parse()?));

    let req = StaticRequest {
        style: "demo".into(),
        camera: Camera::new(-9.14, 38.72, 4.0)?,
        spec: ImageSpec::new(512, 512)?,
    };

    let image = pool.render(req)?;
    image.save("map.png")?;
    println!("Wrote map.png ({}x{})", image.width(), image.height());
    Ok(())
}