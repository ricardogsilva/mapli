//! PyO3 bindings for mapli-core.
//!
//! This crate is deliberatly thin: it converts Python arguments into mapli-core types,
//! releases the GIL around anything slow, and converts results and errors back. All real logic
//! is in mapli-core.

use std::num::NonZeroUsize;
use std::path::PathBuf;

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use mapli_core::{Camera, ImageSpec, PixelRatio, PoolConfig, StaticRequest, Style };

// We use the name `mapli` (and not `mapli-core`) for the Python module because users
// import these from the `mapli` python package
create_exception!(mapli, MapliError, PyException, "Base exception for mapli errors");
create_exception!(mapli, UnknownStyleError, MapliError, "The style id has not been registered");
create_exception!(mapli, StyleLoadError, MapliError, "MapLibre Native failed to load a style.");
create_exception!(mapli, RenderError, MapliError, "MapLibre Native failed render.");


fn to_py_err(err: mapli_core::MapliError) -> PyErr {
    use mapli_core::MapliError as E;
    let msg = err.to_string();
    match err {
        E::InvalidInput(_) => PyValueError::new_err(msg),
        E::UnknownStyle(_) => UnknownStyleError::new_err(msg),
        E::StyleLoadFailed(_) => StyleLoadError::new_err(msg),
        E::RenderFailed(_) | E::PngEncodingFailed(_) => RenderError::new_err(msg),
        _ => MapliError::new_err(msg),
    }
}

/// A pool of MapLibre Native render workers.
///
/// `frozen`: Python only ever gets an immutable reference to the pool, so PyO3 is able to
/// skip runtime borrow tracking, and the object can be shared freely across Python threads.
/// This works because mapli_core::RenderPool is `Send+Sync` and only needs `&self`.
///
/// `subclass`: allows Python subclasses to be created, which is useful for adding async stuff,
/// testing, etc.
#[pyclass(frozen, subclass, module = "mapli._core")]
struct RenderPool {
    inner: mapli_core::RenderPool,
}

#[pymethods]
impl RenderPool {
    #[new]
    #[pyo3(signature = (workers = 1, max_renderers_per_worker = 8))]
    fn new(workers: usize, max_renderers_per_worker: usize) -> PyResult<Self> {
        let workers = NonZeroUsize::new(workers)
            .ok_or_else(|| PyValueError::new_err("workers must be >= 1"))?;
        let max_renderers_per_worker = NonZeroUsize::new(max_renderers_per_worker)
            .ok_or_else(|| PyValueError::new_err("max_renderers_per_worker must be >= 1"))?;
        let inner = mapli_core::RenderPool::new(PoolConfig { workers, max_renderers_per_worker })
            .map_err(to_py_err)?;
        Ok(Self { inner })
    }

    /// Register a style with the pool under `style_id`. Pass exactly one of `url`, `path` or `json`
    #[pyo3(signature = (style_id, *, url=None, path=None, json=None))]
    fn register_style(
        &self,
        style_id: &str,
        url: Option<&str>,
        path: Option<PathBuf>,
        json: Option<String>,
    ) -> PyResult<()> {
        let style = match (url, path, json) {
            (Some(u), None, None) => Style::Url(
                u.parse().map_err(|e| PyValueError::new_err(format!("invalid url: {u:?}: {e}")))?,
            ),
            (None, Some(p), None) => Style::Path(p),
            (None, None, Some(j)) => Style::Json(j),
            _ => return Err(PyValueError::new_err("pass exactly one of url, path or json")),
        };
        self.inner.register_style(style_id, style);
        Ok(())
    }

    fn has_style(&self, style_id: &str) -> bool {
        self.inner.has_style(&style_id.into())
    }

    #[pyo3(signature = (
        style_id, *, lon, lat, zoom ,width, height,
        bearing = 0.0, pitch = 0.0, pixel_ratio = 1.0,
    ))]
    #[allow(clippy::too_many_arguments)]
    fn render<'py>(
        &self,
        py: Python<'py>,
        style_id: &str,
        lon: f64,
        lat: f64,
        zoom: f64,
        width: u32,
        height: u32,
        bearing: f64,
        pitch: f64,
        pixel_ratio: f32,
    ) -> PyResult<Bound<'py, PyBytes>> {
        // build and validate the request while holding the GIL, so we can return a
        // PyErr if something is wrong
        let req = build_static_request(
            style_id, lon, lat, zoom, width, height, bearing, pitch, pixel_ratio
        ).map_err(to_py_err)?;

        // now release the GIL for doing the actual rendering and PNG encoding, since both are
        // slow and don't need it (they don't touch Python objects).
        let png = py
            .detach(|| {
                let image = self.inner.render(req)?;
                mapli_core::encode_png(&image)
            })
            .map_err(to_py_err)?;

        // we have the GIL again: copy the bytes into a Python `bytes` object and return it
        Ok(PyBytes::new(py, &png))
    }

    fn __repr__(&self) -> String {
        "<mapli.RenderPool>".to_string()
    }
}

#[allow(clippy::too_many_arguments)]
fn build_static_request(
    style_id: &str,
    lon: f64,
    lat: f64,
    zoom: f64,
    width: u32,
    height: u32,
    bearing: f64,
    pitch: f64,
    pixel_ratio: f32,
) -> mapli_core::Result<StaticRequest> {
    let camera = Camera::new(lon, lat, zoom)?
        .with_bearing(bearing)?
        .with_pitch(pitch)?;
    let spec = ImageSpec::new(width, height)?
        .with_pixel_ratio(PixelRatio::new(pixel_ratio)?);
    Ok(StaticRequest { style: style_id.into(), camera, spec })
}


/// The function name must match the last segment of `module-name` in the pyproject.toml file
/// and also the `lib[name] in Cargo.toml.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RenderPool>()?;

    let py = m.py();
    m.add("MapliError", py.get_type::<MapliError>())?;
    m.add("UnknownStyleError", py.get_type::<UnknownStyleError>())?;
    m.add("StyleLoadError", py.get_type::<StyleLoadError>())?;
    m.add("RenderError", py.get_type::<RenderError>())?;
    Ok(())
}