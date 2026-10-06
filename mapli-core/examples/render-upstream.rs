//! Render a map using the upstream MapLibre Native library.
//!
//! This example demonstrates how to render a map using the MapLibre Native library in Rust.
//! It creates a static renderer, loads a style from a URL, sets the camera position, and
//! renders the map to an image. Finally, it saves the rendered image as "map.png".

use maplibre_native::{CameraUpdate, ImageRendererBuilder, Image, LatLng};
use std::num::NonZeroU32;

fn main() {
    let mut renderer = ImageRendererBuilder::new()
        .with_size(
            NonZeroU32::new(512).unwrap(),
            NonZeroU32::new(512).unwrap()
        )
        .build_static_renderer();
    renderer.load_style_from_url(&"https://demotiles.maplibre.org/style.json".parse().unwrap());
    let camera = CameraUpdate::new()
        .center(LatLng { lat: 0.0, lng: 0.0 })
        .zoom(0.0);
    let image: Image = renderer.render_static(&camera).unwrap();

    // Access the underlying ImageBuffer for all operations
    let img_buffer = image.as_image();
    println!("Image dimensions: {}x{}", img_buffer.width(), img_buffer.height());
    img_buffer.save("map.png").unwrap();
}