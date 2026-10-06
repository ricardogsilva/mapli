use std::collections::HashMap;
use std::future::Future;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

use crossbeam_channel::Sender;
use image::RgbaImage;

use crate::error::{MapliError, Result};
use crate::types::{StaticRequest, Style, StyleId};
use crate::worker::{self, Command, Reply};

/// Styles shared between the pool (writer) and workers (readers).
///
/// Values are `Arc<Style>` so a worker can clone one out and release the lock
/// before doing the slow work of loading it.
pub(crate) type StyleRegistry = Arc<RwLock<HashMap<StyleId, Arc<Style>>>>;

type ReplyReceiver = oneshot::Receiver<Result<RgbaImage>>;

pub struct PoolConfig {
    pub workers:usize,
    pub max_renderers_per_worker: usize,
}

impl RenderPool {
    pub fn new(config: PoolConfig) -> Result<Self, Error>;
    pub fn register_style(&self, id: impl Into<StyleId>, style: Style);
    pub fn render(&self, req: StaticRequest) -> Result<RgbaImage, Error>;
    pub fn render_async(&self, req: StaticRequest) -> impl Future<Output = Result<RgbaImage, Error>>;

    pub fn render_tile(&self, req: TileRequest) -> Result<RgbaImage, Error>;
    pub fn render_tile_async(&self, req: TileRequest) -> impl Future<Output = Result<RgbaImage, Error>>;
}