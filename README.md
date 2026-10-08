# mapli

Python bindings for the [maplibre_native] Rust crate.

This project provides a simple API to render map images and tiles from Python. It is a thin wrapper around 
the [maplibre_native] crate, which is a Rust implementation of the MapLibre GL Native library.


### Examples


```python
import asyncio
from pathlib import Path
import mapli


async def main() -> None:
    render_pool = mapli.RenderPool(workers=2)
    render_pool.register_style("demo", url="https://demotiles.maplibre.org/style.json")
    png_bytes = await render_pool.arender(
        "demo", lon=-9.14, lat=38.72, zoom=5, width=256, height=256
    )
    Path(f"mapli-lisbon.png").write_bytes(png_bytes)
    print(f"wrote mapli-lisbon.png ({len(png_bytes)} bytes)")


if __name__ == "__main__":
    asyncio.run(main())
```

There are two additional early usage examples in the `/examples` dir:

- A script that concurrently renders map thumbnails for some cities;
- A small FastAPI app that renders map PNGs and serves XYZ map tiles;

### Tiles

Besides static images, mapli can render map tiles in the XYZ (WebMercatorQuad) scheme:

```python
import mapli

pool = mapli.RenderPool(workers=2)
pool.register_style("demo", url="https://demotiles.maplibre.org/style.json")

png = pool.render_tile("demo", z=2, x=1, y=1)  # or `await pool.arender_tile(...)`
```

`tile_size` defaults to 512 px, which is MapLibre's native tile size. Most XYZ clients (Leaflet,
OpenLayers) expect 256 px tiles, which you get with `tile_size=256`. Because MapLibre zoom levels
are based on 512 px tiles, a 256 px tile is drawn one zoom level lower, so 256 px tiles cannot be
rendered at `z=0`. Use `pixel_ratio=2` for high-DPI ("@2x") tiles.


## System requirements

> [!IMPORTANT] OS support
>
> Linux (x86_64, ARM64) is supported. macOS on Apple Silicon should also be possible via MapLibre Native's Metal 
> backend but is not yet built or tested. 
> Windows is not supported and mapli will only be able to enable support for it if the 
> [upstream maplibre-native-rs] project ever decides to implement it.


[upstream maplibre-native-rs]: https://github.com/maplibre/maplibre-native-rs#platform-support

mapli renders with MapLibre Native's OpenGL backend, which on Linux creates an EGL context in the
process. 

At runtime, mapli needs both:

- An EGL driver - A GPU is not required: Mesa's `llvmpipe` software renderer works fine on
  servers and in containers. On Debian/Ubuntu:

  ```shell
  sudo apt-get install libegl1 libegl-mesa0 libgl1-mesa-dri
  ```

- A display - This can be your normal display (on a desktop machine) or a headless server, 
  like xvfb

  ```shell
  # only needed if there is no graphical display installed
  sudo apt-get install xvfb
  xvfb-run -a python your_app.py
  ```

  For long-running services, you can instead run `Xvfb` as its own service and set the `DISPLAY`
  environment variable for the process using mapli. Desktop machines that already have a display
  don't need this.

If either is missing, creating a `RenderPool` raises `mapli.GraphicsUnavailableError` with details
about which EGL step failed:

```python
import mapli

try:
    pool = mapli.RenderPool(workers=2)
except mapli.GraphicsUnavailableError as exc:
    print(f"cannot render on this machine: {exc}")
```


## Development

### rust crates

Use `cargo` to build, test, show docs, etc. as usual.


Building mapli compiles MapLibre Native from source, which needs a C++ toolchain and several
development packages. On Debian/Ubuntu, ensure you have these installed:

```shell
sudo apt-get install \
    build-essential \
    ccache \
    cmake \
    glslang-dev \
    glslang-tools \
    libcurl4-openssl-dev \
    libegl1-mesa-dev \
    libfontconfig-dev \
    libgl1-mesa-dev \
    libgl1-mesa-dri \
    libicu-dev \
    libjpeg-turbo8-dev \
    libpng-dev \
    libuv1-dev \
    libwebp-dev
    libz-dev \
    pkg-config
```

The first build takes a while (around 15 minutes on a GitHub Actions runner). Setting
`MLN_CMAKE_CXX_LAUNCHER=ccache` routes the C++ compilation through ccache, which makes rebuilds
much faster.


### Python package

The repo contains a Python package that uses the rust crates. Checkout the `pyproject.toml` file for more info
on how it is set up. In short, it makes use of [maturin] as the build system for the rust crate, and uses [uv]
as a build tool for the Python package.

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
