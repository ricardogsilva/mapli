# mapli

Python bindings for the [maplibre_native] Rust crate.

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

[maplibre_native]: https://docs.rs/maplibre_native/0.10.0/maplibre_native/index.html
[maturin]: https://www.maturin.rs/
[uv]: https://docs.astral.sh/uv/
