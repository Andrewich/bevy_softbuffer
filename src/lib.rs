// pub mod prelude {
//     pub use crate::{SoftBufferOptions, PixelsPlugin, PixelsResource, PixelsStage};
// }

pub use softbuffer;

use bevy::{
    diagnostic::{Diagnostic, DiagnosticId, Diagnostics},
    prelude::*,
    window::{WindowBackendScaleFactorChanged, WindowId, WindowResized},
    winit::WinitWindows,
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use softbuffer::GraphicsContext;
use std::time::Instant;
use std::sync::Arc;
use std::cell::{Ref, RefCell};

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum SoftBufferStage {
    Draw,
    Render,
}

#[derive(Debug, Clone)]
pub struct SoftBufferOptions {
    /// Width of the pixel buffer
    pub width: u32,
    /// Height of the pixel buffer
    pub height: u32,
}

impl Default for SoftBufferOptions {
    fn default() -> Self {
        SoftBufferOptions {
            width: 180,
            height: 120,
        }
    }
}

#[derive(Default)]
pub struct Device<W: HasRawWindowHandle> {
    context: RefCell<Option<GraphicsContext<W>>>,
}

//impl Device {
    // pub fn get_context(&self) -> std::cell::Ref<web_sys::WebGl2RenderingContext> {
    //     return Ref::map(self.context.borrow(), |t| {
    //         t.as_ref().expect("webgl context is set")
    //     });
    // }

    // pub fn set_context(&self, context: web_sys::WebGl2RenderingContext) {
    //     *self.context.borrow_mut() = Some(context);
    // }
//}

// impl std::fmt::Debug for Device {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&format!("Device: {:?}", self.context.borrow()))
//     }
// }

unsafe impl Send for Device<W: HasRawWindowHandle> {}
unsafe impl Sync for Device<W: HasRawWindowHandle> {}


pub struct PixelsResource {
    pub device: Arc<Device>,
}