# mapli

Python bindings for the [maplibre_native] Rust crate.

This project provides a simple API to render map images and tiles from Python. It is a thin wrapper around 
the [maplibre_native] crate, which is a Rust implementation of the MapLibre GL Native library.

### Examples

There are two early usage examples in the `/examples` dir:

- A quick script that renders a map thumbnail.
- A small FastAPI app that exposes a single path operation that renders map PNGs


## rust crate

Use `cargo` to build, test, show docs, etc. as usual.

## Python package

The repo contains a Python package that uses the rust crate. Checkout the `pyproject.toml` file for more info
on how it is set up. In short, it makes use of [maturin] as the build system for the rust crate, and uses [uv]
as a build tool for the Python package.

### development

`uv sync` builds the project with a development build of the `mapli` crate. This works because we have
`editable-profile = "dev"` in the `[tool.maturin]` section in `pyproject.toml`. The default sync way of uv is to
perform an editable build of the project.

This means we can simply invoke `uv` as usual.

Running `uv sync --no-editable --reinstall-package` causes uv to build a non-debug version of the package, which can
be useful for benchmarking.

## Related projects

Several other Python packages integrate with MapLibre, at different layers. 

[py-maplibregl] (`maplibre` on PyPI) generates HTML/JavaScript that runs MapLibre GL JS in a browser, with Jupyter 
and Shiny integrations; it targets interactive client-side maps and does not produce raster output in Python. 

[leafmap] and [lonboard] follow the same client-side model (leafmap includes a MapLibre backend built on 
py-maplibregl; lonboard renders with deck.gl over a MapLibre basemap). 

For server-side rendering, [pymgl] provides in-process nanobind bindings to a C++ wrapper around
MapLibre Native for rendering styles to PNG. It ships wheels for Linux (requiring Xvfb) and macOS arm64, and its API
is documented as subject to change. 
[mlnative] uses the [maplibre_native] crate in a separate renderer binary, driven from Python as a long-lived 
subprocess over stdin/stdout; this isolates the renderer from the Python process at the cost of serializing each 
request and response across the process boundary, and wheels are Linux-only.
[maplibre-native-ffi] is a MapLibre project that defines a C API over MapLibre Native, with bindings for several
languages. Its Python bindings are a PyO3 extension over that C API, exposing a broad, low-level surface (the host
manages the graphics context), and the project is pre-1.0 with an unstable ABI. 

Out-of-process HTTP services such as [tileserver-gl] (Node.js, rasterizing with MapLibre
Native) are another option, which adds a separate service to deploy. 

mapli binds the same [maplibre_native] crate in-process through PyO3, exposing a pooled renderer
with configurable worker threads, renderer reuse per style, and both synchronous and native `async` render calls. 
Compared with the subprocess and HTTP approaches, it avoids IPC overhead. Compared with the FFI bindings, it exposes 
a narrower, task-specific API.

[maplibre_native]: https://docs.rs/maplibre_native/0.10.0/maplibre_native/index.html
[maturin]: https://www.maturin.rs/
[uv]: https://docs.astral.sh/uv/
[py-maplibregl]: https://github.com/eodaGmbH/py-maplibregl
[leafmap]: https://github.com/opengeos/leafmap
[lonboard]: https://github.com/developmentseed/lonboard
[pymgl]: https://github.com/brendan-ward/pymgl
[tileserver-gl]: https://github.com/maptiler/tileserver-gl
[mlnative]: https://github.com/adonm/mlnative
[maplibre-native-ffi]: https://github.com/maplibre/maplibre-native-ffi
