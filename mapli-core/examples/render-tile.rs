//! Render a map tile using the mapli-core library.
//!
//! This example registers a style from a URL and renders tile 2/1/1 twice: once as a 512 px tile
//! (MapLibre's native tile size), and once as a 256 px tile at a pixel ratio of 2 (a "@2x" tile
//! for classic XYZ clients). The results are saved as "tile-512.png" and "tile-256@2x.png".

use std::num::NonZeroU32;

use mapli_core::{OutputFormat, PixelRatio, PoolConfig, RenderPool, Style, TileCoord, TileRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = RenderPool::new(PoolConfig::default())?;
    pool.register_style(
        "demo",
        Style::Url("https://demotiles.maplibre.org/style.json".parse()?),
    );

    let tile = TileCoord::new(2, 1, 1)?;
    for (tile_size, pixel_ratio, path) in
        [(512, 1.0, "tile-512.png"), (256, 2.0, "tile-256@2x.png")]
    {
        let req = TileRequest {
            style: "demo".into(),
            tile,
            tile_size: NonZeroU32::new(tile_size).unwrap(),
            pixel_ratio: PixelRatio::new(pixel_ratio)?,
            format: OutputFormat::Png,
        };
        let png = pool.render_tile(req)?.into_png()?;
        std::fs::write(path, &png)?;
        println!("Wrote {path} ({} bytes)", png.len());
    }
    Ok(())
}
