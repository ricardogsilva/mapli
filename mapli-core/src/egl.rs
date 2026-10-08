//! Pre-flight check that the system can provide the EGL context MapLibre Native renders with.
//!
//! MapLibre Native's Linux headless backend creates its EGL context lazily, inside its run loop,
//! on the first render. If that fails, it throws a C++ exception that cannot be caught from Rust,
//! and the whole process aborts. This module repeats the same EGL setup steps (see
//! `platform/linux/src/headless_backend_egl.cpp` in maplibre-native) up front, so that a missing
//! driver or display is reported as a regular error when the pool is created.
//!
//! The check runs once per process and its result is cached.

use std::ffi::c_void;
use std::ptr;
use std::sync::OnceLock;

use crate::error::{MapliError, Result};

type EGLDisplay = *mut c_void;
type EGLConfig = *mut c_void;
type EGLContext = *mut c_void;
type EGLSurface = *mut c_void;
type EGLBoolean = u32;
type EGLenum = u32;
type EGLint = i32;

const EGL_DEFAULT_DISPLAY: *mut c_void = ptr::null_mut();
const EGL_NO_DISPLAY: EGLDisplay = ptr::null_mut();
const EGL_NO_CONTEXT: EGLContext = ptr::null_mut();
const EGL_NO_SURFACE: EGLSurface = ptr::null_mut();
const EGL_TRUE: EGLint = 1;
const EGL_NONE: EGLint = 0x3038;
const EGL_OPENGL_ES_API: EGLenum = 0x30A0;
const EGL_RENDERABLE_TYPE: EGLint = 0x3040;
const EGL_OPENGL_ES3_BIT: EGLint = 0x0040;
const EGL_SURFACE_TYPE: EGLint = 0x3033;
const EGL_PBUFFER_BIT: EGLint = 0x0001;
const EGL_CONTEXT_CLIENT_VERSION: EGLint = 0x3098;
const EGL_WIDTH: EGLint = 0x3057;
const EGL_HEIGHT: EGLint = 0x3056;
const EGL_LARGEST_PBUFFER: EGLint = 0x3058;

#[link(name = "EGL")]
unsafe extern "C" {
    fn eglGetDisplay(display_id: *mut c_void) -> EGLDisplay;
    fn eglInitialize(dpy: EGLDisplay, major: *mut EGLint, minor: *mut EGLint) -> EGLBoolean;
    fn eglBindAPI(api: EGLenum) -> EGLBoolean;
    fn eglChooseConfig(
        dpy: EGLDisplay,
        attrib_list: *const EGLint,
        configs: *mut EGLConfig,
        config_size: EGLint,
        num_config: *mut EGLint,
    ) -> EGLBoolean;
    fn eglCreateContext(
        dpy: EGLDisplay,
        config: EGLConfig,
        share_context: EGLContext,
        attrib_list: *const EGLint,
    ) -> EGLContext;
    fn eglCreatePbufferSurface(
        dpy: EGLDisplay,
        config: EGLConfig,
        attrib_list: *const EGLint,
    ) -> EGLSurface;
    fn eglDestroySurface(dpy: EGLDisplay, surface: EGLSurface) -> EGLBoolean;
    fn eglDestroyContext(dpy: EGLDisplay, ctx: EGLContext) -> EGLBoolean;
    fn eglGetError() -> EGLint;
}

/// Verify that an EGL context suitable for MapLibre Native can be created.
pub(crate) fn check() -> Result<()> {
    static RESULT: OnceLock<Result<(), String>> = OnceLock::new();
    RESULT
        .get_or_init(probe)
        .clone()
        .map_err(MapliError::GraphicsUnavailable)
}

fn probe() -> Result<(), String> {
    let failed = |step: &str| {
        // SAFETY: eglGetError takes no arguments and only reads thread-local EGL state
        let code = unsafe { eglGetError() };
        format!("{step} failed (EGL error 0x{code:04X})")
    };

    // SAFETY: all calls follow the EGL API contract: the display is checked before use, the
    // attribute lists are EGL_NONE-terminated and outlive the calls, and every handle we create
    // is destroyed before returning.
    unsafe {
        let display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
        if display == EGL_NO_DISPLAY {
            return Err("eglGetDisplay() returned no display".into());
        }
        // The display is deliberately never terminated. EGL does not reference-count
        // initialization, so `eglTerminate` would also tear down the display that MapLibre Native
        // shares process-wide if it is already rendering. Initializing an initialized display is a
        // no-op, so leaving it initialized is harmless.
        if eglInitialize(display, ptr::null_mut(), ptr::null_mut()) == 0 {
            return Err(failed("eglInitialize()"));
        }
        if eglBindAPI(EGL_OPENGL_ES_API) == 0 {
            return Err(failed("eglBindAPI(EGL_OPENGL_ES_API)"));
        }

        let config_attribs = [
            EGL_RENDERABLE_TYPE,
            EGL_OPENGL_ES3_BIT,
            EGL_SURFACE_TYPE,
            EGL_PBUFFER_BIT,
            EGL_NONE,
        ];
        let mut config: EGLConfig = ptr::null_mut();
        let mut num_configs: EGLint = 0;
        if eglChooseConfig(
            display,
            config_attribs.as_ptr(),
            &mut config,
            1,
            &mut num_configs,
        ) == 0
            || num_configs != 1
        {
            return Err(failed(
                "eglChooseConfig() for an OpenGL ES 3 pbuffer config",
            ));
        }

        let context_attribs = [EGL_CONTEXT_CLIENT_VERSION, 3, EGL_NONE];
        let context = eglCreateContext(display, config, EGL_NO_CONTEXT, context_attribs.as_ptr());
        if context == EGL_NO_CONTEXT {
            return Err(failed("eglCreateContext() for OpenGL ES 3"));
        }

        let surface_attribs = [
            EGL_WIDTH,
            8,
            EGL_HEIGHT,
            8,
            EGL_LARGEST_PBUFFER,
            EGL_TRUE,
            EGL_NONE,
        ];
        let surface = eglCreatePbufferSurface(display, config, surface_attribs.as_ptr());
        let result = if surface == EGL_NO_SURFACE {
            Err(failed("eglCreatePbufferSurface()"))
        } else {
            eglDestroySurface(display, surface);
            Ok(())
        };
        eglDestroyContext(display, context);
        result
    }
}
